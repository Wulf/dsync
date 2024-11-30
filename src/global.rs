use std::collections::HashMap;

/// Available options for string types
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum StringType {
    /// Use `String`
    #[default]
    String,
    /// Use `&str`
    Str,
    /// Use `Cow<str>`
    Cow,
}

impl StringType {
    /// Get the current [StringType] as a rust type string
    pub fn as_str(&self) -> &'static str {
        match self {
            StringType::String => "String",
            StringType::Str => "&'a str",
            StringType::Cow => "Cow<'a, str>",
        }
    }

    /// Get the lifetime used for the current [StringType]
    pub fn get_lifetime(&self) -> &'static str {
        match self {
            StringType::String => "",
            StringType::Str => "'a",
            StringType::Cow => "'a",
        }
    }
}

/// Available options for bytes types
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BytesType {
    /// Use `Vec<u8>`
    #[default]
    Vec,
    /// Use `&[u8]`
    Slice,
    /// Use `Cow<[u8]>`
    Cow,
}

impl BytesType {
    /// Get the current [BytesType] as a rust type string
    pub fn as_str(&self) -> &'static str {
        match self {
            BytesType::Vec => "Vec<u8>",
            BytesType::Slice => "&'a [u8]",
            BytesType::Cow => "Cow<'a, [u8]>",
        }
    }

    /// Get the lifetime used for the current [BytesType]
    pub fn get_lifetime(&self) -> &'static str {
        match self {
            BytesType::Vec => "",
            BytesType::Slice => "'a",
            BytesType::Cow => "'a",
        }
    }
}

/// Options for a individual table
#[derive(Debug, Clone)]
pub struct TableOptions<'a> {
    /// Ignore a specific table
    ignore: Option<bool>,
    /// Names used for autogenerated columns which are NOT primary keys (for example: `created_at`, `updated_at`, etc.).
    autogenerated_columns: Option<Vec<&'a str>>,

    #[cfg(feature = "tsync")]
    /// Adds #[tsync] attribute to structs (see <https://github.com/Wulf/tsync>)
    tsync: Option<bool>,

    #[cfg(feature = "async")]
    /// Uses diesel_async for generated functions (see <https://github.com/weiznich/diesel_async>)
    use_async: Option<bool>,

    /// Generates `serde::Serialize` and `serde::Deserialize` derive implementations
    use_serde: bool,

    /// Generates the CRUD functions for generated models
    fns: bool,

    /// Determines which string type to use for Create* structs
    create_str_type: StringType,

    /// Determines which string type to use for Update* structs
    update_str_type: StringType,

    /// Determines which bytes type to use for Create* structs
    create_bytes_type: BytesType,

    /// Determines which bytes type to use for Update* structs
    update_bytes_type: BytesType,

    /// Only Generate a single model file instead of a directory with "mod.rs" and "generated.rs"
    single_model_file: bool,

    /// Indicates this table is meant to be read-only (don't generate Update & Create structs)
    read_only: bool,
}

impl<'a> TableOptions<'a> {
    #[inline]
    pub fn get_ignore(&self) -> bool {
        self.ignore.unwrap_or_default()
    }

    #[inline]
    #[cfg(feature = "tsync")]
    pub fn get_tsync(&self) -> bool {
        self.tsync.unwrap_or_default()
    }

    #[inline]
    #[cfg(feature = "async")]
    pub fn get_async(&self) -> bool {
        self.use_async.unwrap_or_default()
    }

    #[inline]
    pub fn get_serde(&self) -> bool {
        self.use_serde
    }

    #[inline]
    pub fn get_fns(&self) -> bool {
        self.fns
    }

    #[inline]
    pub fn get_create_str_type(&self) -> StringType {
        self.create_str_type
    }

    #[inline]
    pub fn get_update_str_type(&self) -> StringType {
        self.update_str_type
    }

    #[inline]
    pub fn get_create_bytes_type(&self) -> BytesType {
        self.create_bytes_type
    }

    #[inline]
    pub fn get_update_bytes_type(&self) -> BytesType {
        self.update_bytes_type
    }

    #[inline]
    pub fn get_autogenerated_columns(&self) -> &[&'_ str] {
        self.autogenerated_columns.as_deref().unwrap_or_default()
    }

    #[inline]
    pub fn get_readonly(&self) -> bool {
        self.read_only
    }

    #[inline]
    pub fn get_single_model_file(&self) -> bool {
        self.single_model_file
    }

    #[inline]
    pub fn ignore(self) -> Self {
        Self {
            ignore: Some(true),
            ..self
        }
    }

    #[inline]
    #[cfg(feature = "tsync")]
    pub fn tsync(self) -> Self {
        Self {
            tsync: Some(true),
            ..self
        }
    }

    #[inline]
    #[cfg(feature = "async")]
    pub fn use_async(self) -> Self {
        Self {
            use_async: Some(true),
            ..self
        }
    }

    #[inline]
    pub fn disable_serde(self) -> Self {
        Self {
            use_serde: false,
            ..self
        }
    }

    #[inline]
    pub fn disable_fns(self) -> Self {
        Self { fns: false, ..self }
    }

    #[inline]
    pub fn single_model_file(self) -> Self {
        Self {
            single_model_file: true,
            ..self
        }
    }

    #[inline]
    pub fn autogenerated_columns(self, cols: Vec<&'a str>) -> Self {
        Self {
            autogenerated_columns: Some(cols),
            ..self
        }
    }

    #[inline]
    pub fn create_str_type(self, type_: StringType) -> Self {
        Self {
            create_str_type: type_,
            ..self
        }
    }

    #[inline]
    pub fn update_str_type(self, type_: StringType) -> Self {
        Self {
            update_str_type: type_,
            ..self
        }
    }

    #[inline]
    pub fn create_bytes_type(self, type_: BytesType) -> Self {
        Self {
            create_bytes_type: type_,
            ..self
        }
    }

    #[inline]
    pub fn update_bytes_type(self, type_: BytesType) -> Self {
        Self {
            update_bytes_type: type_,
            ..self
        }
    }

    #[inline]
    pub fn set_read_only(&mut self, value: bool) {
        self.read_only = value;
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
            create_str_type: other.create_str_type,
            update_str_type: other.update_str_type,
            create_bytes_type: other.create_bytes_type,
            update_bytes_type: other.update_bytes_type,
            single_model_file: self.single_model_file || other.single_model_file,
            read_only: self.read_only || other.read_only,
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
            create_str_type: Default::default(),
            update_str_type: Default::default(),
            create_bytes_type: Default::default(),
            update_bytes_type: Default::default(),
            single_model_file: false,
            read_only: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenerationConfigOpts<'a> {
    /// Specific Table options for a given table
    pub table_options: HashMap<&'a str, TableOptions<'a>>,
    /// Default table options, used when not in `table_options`
    pub default_table_options: TableOptions<'a>,
    /// Diesel schema import path
    ///
    /// by default `crate::schema::`
    pub schema_path: String,
    /// Dsync model import path
    ///
    /// by default `crate::models::`
    pub model_path: String,
    /// Generate common structs only once in a "common.rs" file
    pub once_common_structs: bool,
    /// Generate the "ConnectionType" type only once in a "common.rs" file
    pub once_connection_type: bool,
    /// Prefixes to treat tables as readonly
    pub readonly_prefixes: Vec<String>,
    /// Suffixes to treat tables as readonly
    pub readonly_suffixes: Vec<String>,
    /// Generate the "default" function in an `impl Default`
    pub default_impl: bool,
}

impl GenerationConfigOpts<'_> {
    pub fn table(&self, name: &str) -> TableOptions<'_> {
        let table = self
            .table_options
            .get(name)
            .unwrap_or(&self.default_table_options);

        let mut table = table.apply_defaults(&self.default_table_options);

        if self.readonly_prefixes.iter().any(|v| name.starts_with(v))
            || self.readonly_suffixes.iter().any(|v| name.ends_with(v))
        {
            table.set_read_only(true);
        }

        table
    }
}

pub const DEFAULT_SCHEMA_PATH: &str = "crate::schema::";
pub const DEFAULT_MODEL_PATH: &str = "crate::models::";

impl Default for GenerationConfigOpts<'_> {
    fn default() -> Self {
        Self {
            table_options: HashMap::default(),
            default_table_options: Default::default(),
            schema_path: String::from(DEFAULT_SCHEMA_PATH),
            model_path: String::from(DEFAULT_MODEL_PATH),
            once_common_structs: false,
            once_connection_type: false,
            readonly_prefixes: Vec::default(),
            readonly_suffixes: Vec::default(),
            default_impl: false,
        }
    }
}

/// Global config, not table specific
#[derive(Debug, Clone)]
pub struct GenerationConfig<'a> {
    /// Connection type to insert
    ///
    /// For example:
    /// - `diesel::pg::PgConnection`
    /// - `diesel::sqlite::SqliteConnection`
    /// - `diesel::mysql::MysqlConnection`
    /// - `diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>`
    /// - or, your custom diesel connection type (struct which implements `diesel::connection::Connection`)
    pub connection_type: String,

    #[cfg(feature = "advanced-queries")]
    /// Diesel backend
    ///
    /// For example:
    /// - `diesel::pg::Pg` (default)
    /// - `diesel::sqlite::Sqlite`
    /// - `diesel::mysql::Mysql`
    /// - or, your custom diesel backend type (struct which implements `diesel::backend::Backend`)
    pub diesel_backend: String,

    /// Optional Options
    /// ```
    /// # use dsync::{GenerationConfig,GenerationConfigOpts};
    /// GenerationConfig {
    ///  // ... all required options
    ///  # connection_type: String::default(),
    ///  options: Default::default(),
    /// };
    /// // or
    /// GenerationConfig {
    ///  // ... all required options
    ///  # connection_type: String::default(),
    ///  options: GenerationConfigOpts {
    ///    ..Default::default()
    ///  },
    /// };
    /// ```
    pub options: GenerationConfigOpts<'a>,
}

impl<'a> GenerationConfig<'a> {
    #[cfg(not(feature = "advanced-queries"))]
    /// Create a new Instance with default [GenerationConfigOpts]
    ///
    /// Builder
    pub fn new<C: Into<String>>(connection_type: C) -> Self {
        Self {
            connection_type: connection_type.into(),
            options: GenerationConfigOpts::default(),
        }
    }

    #[cfg(feature = "advanced-queries")]
    /// Create a new Instance with default [GenerationConfigOpts]
    ///
    /// Builder
    pub fn new<C: Into<String>, B: Into<String>>(connection_type: C, diesel_backend: B) -> Self {
        Self {
            connection_type: connection_type.into(),
            diesel_backend: diesel_backend.into(),
            options: GenerationConfigOpts::default(),
        }
    }

    /// Replace the options with the new options
    ///
    /// Builder
    #[inline]
    pub fn with_options(mut self, options: GenerationConfigOpts<'a>) -> Self {
        self.options = options;

        self
    }

    // Wrapper for [GenerationConfigOpts::table()]
    #[inline]
    pub fn table(&self, name: &str) -> TableOptions<'_> {
        self.options.table(name)
    }

    #[inline]
    pub fn get_schema_path(&self) -> &str {
        &self.options.schema_path
    }

    #[inline]
    pub fn get_model_path(&self) -> &str {
        &self.options.model_path
    }

    #[inline]
    pub fn get_once_common_structs(&self) -> bool {
        self.options.once_common_structs
    }

    #[inline]
    pub fn get_once_connection_type(&self) -> bool {
        self.options.once_connection_type
    }

    #[inline]
    pub fn get_default_table_options(&self) -> &TableOptions<'_> {
        &self.options.default_table_options
    }

    /// Get if any of the "once-*" options is active / if the common-file is active
    #[inline]
    pub fn any_once_option(&self) -> bool {
        self.get_once_common_structs() || self.get_once_connection_type()
    }
}

pub fn validate_config(config: &GenerationConfig) -> crate::Result<()> {
    use crate::error::{Error, ErrorEnum};

    // NOTE: the following arrays should likely be joined at compile-time instead of at runtime, but rust does not provide such a thing in std

    #[cfg(feature = "advanced-queries")]
    {
        const VALID_BACKENDS: [&str; 3] = [
            "diesel::pg::Pg",
            "diesel::sqlite::Sqlite",
            "diesel::mysql::Mysql",
        ];

        if config.diesel_backend.is_empty() {
            return Err(Error::new(ErrorEnum::InvalidGenerationConfig(format!(
                "Invalid diesel_backend '{}', please use one of the following: {:?}; or, a custom diesel backend type (a struct which implements `diesel::backend::Backend`).",
                &config.diesel_backend,
                VALID_BACKENDS.join(", ")
            ))));
        }
    }

    const KNOWN_CONNECTIONS: [&str; 4] = [
        "diesel::pg::PgConnection",
        "diesel::sqlite::SqliteConnection",
        "diesel::mysql::MysqlConnection",
        "diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<_>>",
    ];

    if config.connection_type.is_empty() {
        return Err(Error::new(ErrorEnum::InvalidGenerationConfig(format!(
            "option \"connection_type\" cannot be empty. Known Connections types are:\n[{}]",
            KNOWN_CONNECTIONS.join(", ")
        ))));
    }

    Ok(())
}
