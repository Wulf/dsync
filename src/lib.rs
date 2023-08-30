mod code;
pub mod error;
mod file;
mod parser;

use error::IOErrorToError;
pub use error::{Error, Result};
use file::MarkedFile;
use parser::ParsedTableMacro;
pub use parser::FILE_SIGNATURE;
use std::collections::HashMap;
use std::path::PathBuf;

/// Options for a individual table
#[derive(Debug, Clone)]
pub struct TableOptions<'a> {
    /// Ignore a specific table
    ignore: Option<bool>,
    /// Names used for autogenerated columns which are NOT primary keys (for example: `created_at`, `updated_at`, etc.).
    autogenerated_columns: Option<Vec<&'a str>>,

    #[cfg(feature = "tsync")]
    /// Adds #[tsync] attribute to structs (see https://github.com/Wulf/tsync)
    tsync: Option<bool>,

    #[cfg(feature = "async")]
    /// Uses diesel_async for generated functions (see https://github.com/weiznich/diesel_async)
    use_async: Option<bool>,

    /// Generates `serde::Serialize` and `serde::Deserialize` derive implementations
    use_serde: bool,

    /// Generates the CRUD functions for generated models
    fns: bool,
}

impl<'a> TableOptions<'a> {
    pub fn get_ignore(&self) -> bool {
        self.ignore.unwrap_or_default()
    }

    #[cfg(feature = "tsync")]
    pub fn get_tsync(&self) -> bool {
        self.tsync.unwrap_or_default()
    }

    #[cfg(feature = "async")]
    pub fn get_async(&self) -> bool {
        self.use_async.unwrap_or_default()
    }

    pub fn get_serde(&self) -> bool {
        self.use_serde
    }

    pub fn get_fns(&self) -> bool {
        self.fns
    }

    pub fn get_autogenerated_columns(&self) -> &[&'_ str] {
        self.autogenerated_columns.as_deref().unwrap_or_default()
    }

    pub fn ignore(self) -> Self {
        Self {
            ignore: Some(true),
            ..self
        }
    }

    #[cfg(feature = "tsync")]
    pub fn tsync(self) -> Self {
        Self {
            tsync: Some(true),
            ..self
        }
    }

    #[cfg(feature = "async")]
    pub fn use_async(self) -> Self {
        Self {
            use_async: Some(true),
            ..self
        }
    }

    pub fn disable_serde(self) -> Self {
        Self {
            use_serde: false,
            ..self
        }
    }

    pub fn disable_fns(self) -> Self {
        Self { fns: false, ..self }
    }

    pub fn autogenerated_columns(self, cols: Vec<&'a str>) -> Self {
        Self {
            autogenerated_columns: Some(cols.clone()),
            ..self
        }
    }

    /// Fills any `None` properties with values from another TableConfig
    pub fn apply_defaults(&self, other: &TableOptions<'a>) -> Self {
        Self {
            ignore: self.ignore.or(other.ignore),
            #[cfg(feature = "tsync")]
            tsync: self.tsync.or(other.tsync),
            #[cfg(feature = "async")]
            use_async: self.use_async.or(other.use_async),
            autogenerated_columns: self
                .autogenerated_columns
                .clone()
                .or_else(|| other.autogenerated_columns.clone()),

            use_serde: self.use_serde || other.use_serde,
            fns: self.fns || other.fns,
        }
    }
}

impl<'a> Default for TableOptions<'a> {
    fn default() -> Self {
        Self {
            ignore: Default::default(),
            autogenerated_columns: Default::default(),
            #[cfg(feature = "tsync")]
            tsync: Default::default(),
            #[cfg(feature = "async")]
            use_async: Default::default(),
            use_serde: true,
            fns: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenerationConfig<'a> {
    /// Specific Table options for a given table
    pub table_options: HashMap<&'a str, TableOptions<'a>>,
    /// Default table options, used when not in `table_options`
    pub default_table_options: TableOptions<'a>,
    /// Connection type to insert
    ///
    /// Example: `diesel::SqliteConnection`
    pub connection_type: String,
    /// Diesel schema import path
    ///
    /// by default `crate::schema::`
    pub schema_path: String,
    /// Dsync model import path
    ///
    /// by default `crate::models::`
    pub model_path: String,
}

impl GenerationConfig<'_> {
    pub fn table(&self, name: &str) -> TableOptions<'_> {
        let t = self
            .table_options
            .get(name)
            .unwrap_or(&self.default_table_options);

        t.apply_defaults(&self.default_table_options)
    }
}

/// Generate a model for the given schema contents
///
/// Model is returned and not saved to disk yet
pub fn generate_code(
    diesel_schema_file_contents: String,
    config: GenerationConfig,
) -> Result<Vec<ParsedTableMacro>> {
    parser::parse_and_generate_code(diesel_schema_file_contents, &config)
}

/// Generate all Models for a given diesel schema file
///
/// Models are saved to disk
pub fn generate_files(
    input_diesel_schema_file: PathBuf,
    output_models_dir: PathBuf,
    config: GenerationConfig,
) -> Result<()> {
    let input = input_diesel_schema_file;
    let output_dir = output_models_dir;

    let generated = generate_code(
        std::fs::read_to_string(&input).attach_path_err(&input)?,
        config,
    )?;

    if !output_dir.exists() {
        std::fs::create_dir(&output_dir).attach_path_err(&output_dir)?;
    } else if !output_dir.is_dir() {
        return Err(Error::not_a_directory(
            "Expected output argument to be a directory or non-existent.",
            output_dir,
        ));
    }

    // check that the mod.rs file exists
    let mut mod_rs = MarkedFile::new(output_dir.join("mod.rs"))?;

    // pass 1: add code for new tables
    for table in generated.iter() {
        let table_dir = output_dir.join(table.name.to_string());

        if !table_dir.exists() {
            std::fs::create_dir(&table_dir).attach_path_err(&table_dir)?;
        }

        if !table_dir.is_dir() {
            return Err(Error::not_a_directory("Expected a directory", table_dir));
        }

        let mut table_generated_rs = MarkedFile::new(table_dir.join("generated.rs"))?;
        let mut table_mod_rs = MarkedFile::new(table_dir.join("mod.rs"))?;

        table_generated_rs.ensure_file_signature()?;
        table_generated_rs.file_contents = table.generated_code.clone();
        table_generated_rs.write()?;

        table_mod_rs.ensure_mod_stmt("generated");
        table_mod_rs.ensure_use_stmt("generated::*");
        table_mod_rs.write()?;

        mod_rs.ensure_mod_stmt(table.name.to_string().as_str());
    }

    // pass 2: delete code for removed tables
    for item in std::fs::read_dir(&output_dir).attach_path_err(&output_dir)? {
        let item = item.attach_path_err(&output_dir)?;

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
            g.name
                .to_string()
                .eq_ignore_ascii_case(associated_table_name)
        });
        if found.is_some() {
            continue;
        }

        // this table was deleted, let's delete the generated code
        std::fs::remove_file(&generated_rs_path).attach_path_err(&generated_rs_path)?;

        // remove the mod.rs file if there isn't anything left in there except the use stmt
        let table_mod_rs_path = item.path().join("mod.rs");
        if table_mod_rs_path.exists() {
            let mut table_mod_rs = MarkedFile::new(table_mod_rs_path)?;

            table_mod_rs.remove_mod_stmt("generated");
            table_mod_rs.remove_use_stmt("generated::*");
            table_mod_rs.write()?;

            if table_mod_rs.file_contents.trim().is_empty() {
                table_mod_rs.delete()?;
            } else {
                table_mod_rs.write()?; // write the changes we made above
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

    Ok(())
}
