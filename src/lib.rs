//! dsync library
//!
//! The dsync library allows creating a custom binary for dsync
//!
//! ## Features
//!
//! - `async`: enable support for [diesel_async](https://github.com/weiznich/diesel_async)
//! - `tsync`: enable support for [tsync](https://github.com/Wulf/tsync)
//! - `backtrace`: enable attaching backtraces to dsync errors
//! - `derive-queryablebyname`: enable `diesel::QueryableByName` derives on READ structs
//! - `advanced-queries`: enable experimental pagination and filter functions ([examples](https://github.com/Wulf/dsync/tree/a44afdd08f4447e367aa47ecb91fae88b57f8944/test/advanced_queries))
//!
//! default features: `tsync`, `backtrace`, `derive-queryablebyname`

mod code;
pub mod error;
mod file;
mod global;
mod parser;

pub use global::{
    BytesType, GenerationConfig, GenerationConfigOpts, StringType, TableOptions,
    DEFAULT_MODEL_PATH, DEFAULT_SCHEMA_PATH,
};

use error::IOErrorToError;
pub use error::{Error, Result};
use file::MarkedFile;
use heck::ToSnakeCase;
use parser::ParsedTableMacro;
pub use parser::FILE_SIGNATURE;
use std::fmt::Display;
use std::path::{Path, PathBuf};

/// Generate a model for the given schema contents
///
/// Model is returned and not saved to disk yet
pub fn generate_code(
    diesel_schema_file_contents: &str,
    config: &GenerationConfig,
) -> Result<Vec<ParsedTableMacro>> {
    parser::parse_and_generate_code(diesel_schema_file_contents, config)
}

/// Status indicating what happened to a file
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileChangeStatus {
    /// Status for unchanged file contents
    Unchanged,
    /// Status for modified file contents
    Modified,
    /// Status if the file has been deleted
    Deleted,
}

impl Display for FileChangeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FileChangeStatus::Unchanged => "Unchanged",
                FileChangeStatus::Modified => "Modified",
                FileChangeStatus::Deleted => "Deleted",
            }
        )
    }
}

/// Status indicating what happened to a specific file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileChange {
    /// File in question
    pub file: PathBuf,
    /// Status of the file
    pub status: FileChangeStatus,
}

impl FileChange {
    pub fn new<P: AsRef<std::path::Path>>(path: P, status: FileChangeStatus) -> Self {
        Self {
            file: path.as_ref().to_owned(),
            status,
        }
    }
}

// easily create a [FileChange] from a [MarkedFile]
impl From<&MarkedFile> for FileChange {
    fn from(value: &MarkedFile) -> Self {
        if value.is_modified() {
            Self::new(&value.path, FileChangeStatus::Modified)
        } else {
            Self::new(&value.path, FileChangeStatus::Unchanged)
        }
    }
}

/// Helper function for consistent table module name generation
/// this is used for the rust module path name and for the filename
///
/// input: "tableA", output -> "table_a"
fn get_table_module_name(table_name: &str) -> String {
    table_name.to_snake_case().to_lowercase()
}

/// Generate all Models for a given diesel schema file
///
/// Models are saved to disk
pub fn generate_files(
    input_diesel_schema_file: &Path,
    output_models_dir: &Path,
    config: GenerationConfig,
) -> Result<Vec<FileChange>> {
    #[cfg(feature = "advanced-queries")]
    global::validate_config(&config)?;

    let generated = generate_code(
        &std::fs::read_to_string(input_diesel_schema_file)
            .attach_path_err(input_diesel_schema_file)?,
        &config,
    )?;

    if !output_models_dir.exists() {
        std::fs::create_dir(output_models_dir).attach_path_err(output_models_dir)?;
    } else if !output_models_dir.is_dir() {
        return Err(Error::not_a_directory(
            "Expected output argument to be a directory or non-existent.",
            output_models_dir,
        ));
    }

    // using generated len, because that is very likely the amount (at least) for files
    let mut file_changes = Vec::with_capacity(generated.len());

    // check that the mod.rs file exists
    let mut mod_rs = MarkedFile::new(output_models_dir.join("mod.rs"))?;

    if config.any_once_option() {
        let mut common_file = MarkedFile::new(output_models_dir.join("common.rs"))?;
        common_file.ensure_file_signature()?;
        common_file.change_file_contents({
            let mut tmp = format!("{FILE_SIGNATURE}\n");
            if config.get_once_common_structs() {
                tmp.push_str(&code::generate_common_structs(
                    config.get_default_table_options(),
                ));
            }
            if config.get_once_connection_type() {
                tmp.push('\n');
                tmp.push_str(&code::generate_connection_type(&config));

                // add ending new-line, this should not cause duplicate new-lines because this only gets run if any of the options is set
                // this will need to be refactored if there should ever be more options using common_file
                tmp.push('\n');
            }

            tmp
        });
        common_file.write()?;
        file_changes.push(FileChange::from(&common_file));

        mod_rs.ensure_mod_stmt("common");
    }

    // pass 1: add code for new tables
    for table in generated.iter() {
        if config.get_once_common_structs() && table.name == "common" {
            return Err(Error::other("Cannot have a table named \"common\" while having option \"once_common_structs\" enabled"));
        }
        let table_name = table.name.to_string();
        let table_filename = get_table_module_name(&table_name);
        let table_config = config.table(&table_name);
        let table_dir = if table_config.get_single_model_file() {
            output_models_dir.to_owned()
        } else {
            output_models_dir.join(&table_filename)
        };

        if !table_dir.exists() {
            std::fs::create_dir(&table_dir).attach_path_err(&table_dir)?;
        }

        if !table_dir.is_dir() {
            return Err(Error::not_a_directory("Expected a directory", table_dir));
        }

        let table_file_name = if table_config.get_single_model_file() {
            let mut table_name = table_name;
            table_name.push_str(".rs");
            table_name
        } else {
            "generated.rs".into()
        };

        let mut table_generated_rs = MarkedFile::new(table_dir.join(table_file_name))?;
        let mut table_mod_rs = MarkedFile::new(table_dir.join("mod.rs"))?;

        table_generated_rs.ensure_file_signature()?;
        table_generated_rs.change_file_contents(table.generated_code.clone());
        table_generated_rs.write()?;

        file_changes.push(FileChange::from(&table_generated_rs));

        if !table_config.get_single_model_file() {
            table_mod_rs.ensure_mod_stmt("generated");
            table_mod_rs.ensure_use_stmt("generated::*");
            table_mod_rs.write()?;
            file_changes.push(FileChange::from(&table_mod_rs));
        }

        mod_rs.ensure_mod_stmt(&table_filename);
    }

    // pass 2: delete code for removed tables
    for item in std::fs::read_dir(output_models_dir).attach_path_err(output_models_dir)? {
        // TODO: this does not work with "single-model-file"
        let item = item.attach_path_err(output_models_dir)?;

        // check if item is a directory
        let file_type = item
            .file_type()
            .attach_path_msg(item.path(), "Could not determine type of file")?;
        if !file_type.is_dir() {
            continue;
        }

        // check if it's a generated file
        let generated_rs_path = item.path().join("generated.rs");
        if !generated_rs_path.exists()
            || !generated_rs_path.is_file()
            || !MarkedFile::new(generated_rs_path.clone())?.has_file_signature()
        {
            continue;
        }

        // okay, it's generated, but we need to check if it's for a deleted table
        let file_name = item.file_name();
        let associated_table_name = file_name.to_str().ok_or(Error::other(format!(
            "Could not determine name of file '{:#?}'",
            item.path()
        )))?;
        let found = generated.iter().find(|g| {
            get_table_module_name(&g.name.to_string()).eq_ignore_ascii_case(associated_table_name)
        });
        if found.is_some() {
            continue;
        }

        // this table was deleted, let's delete the generated code
        std::fs::remove_file(&generated_rs_path).attach_path_err(&generated_rs_path)?;
        file_changes.push(FileChange::new(
            &generated_rs_path,
            FileChangeStatus::Deleted,
        ));

        // remove the mod.rs file if there isn't anything left in there except the use stmt
        let table_mod_rs_path = item.path().join("mod.rs");
        if table_mod_rs_path.exists() {
            let mut table_mod_rs = MarkedFile::new(table_mod_rs_path)?;

            table_mod_rs.remove_mod_stmt("generated");
            table_mod_rs.remove_use_stmt("generated::*");
            table_mod_rs.write()?;

            if table_mod_rs.get_file_contents().trim().is_empty() {
                let table_mod_rs = table_mod_rs.delete()?;
                file_changes.push(FileChange::new(table_mod_rs, FileChangeStatus::Deleted));
            } else {
                table_mod_rs.write()?; // write the changes we made above
                file_changes.push(FileChange::from(&table_mod_rs));
            }
        }

        // delete the table dir if there's nothing else in there
        let is_empty = item
            .path()
            .read_dir()
            .attach_path_err(item.path())?
            .next()
            .is_none();
        if is_empty {
            std::fs::remove_dir(item.path()).attach_path_err(item.path())?;
        }

        // remove the module from the main mod_rs file
        mod_rs.remove_mod_stmt(associated_table_name);
    }

    mod_rs.write()?;
    file_changes.push(FileChange::from(&mod_rs));

    Ok(file_changes)
}
