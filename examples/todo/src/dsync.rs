use dsync::{GenerationConfig, TableOptions};
use std::path::PathBuf;

fn main() {
    let dir = env!("CARGO_MANIFEST_DIR");

    let schema_file = PathBuf::from_iter([dir, "src/schema.rs"]);
    let models_dir = PathBuf::from_iter([dir, "src/models"]);

    dsync::generate_files(
        schema_file.as_path(),
        models_dir.as_path(),
        GenerationConfig {
            connection_type: "diesel::pg::PgConnection".into(),
            default_table_options: TableOptions::default().disable_serde().single_model_file(),
            model_path: "crate::models::".into(),
            schema_path: "crate::schema::".into(),
            once_common_structs: true,
            once_connection_type: true,
            readonly_prefixes: vec![],
            readonly_suffixes: vec![],
            table_options: Default::default(),
        },
    )
    .unwrap();
}
