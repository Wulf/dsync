use heck::{ToPascalCase, ToSnakeCase};
use indoc::formatdoc;
use std::borrow::Cow;

use crate::parser::{ParsedColumnMacro, ParsedTableMacro, FILE_SIGNATURE};
use crate::{GenerationConfig, TableOptions};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StructType {
    /// Variant for the `Read` struct which can be queried and has all properties
    Read,
    /// Variant for a `Update*` struct, which has all properties wrapped in [`Option<>`]
    Update,
    /// Variant for a `Create` struct, which only has all the properties which are not autogenerated
    Create,
}

impl StructType {
    /// Get the prefix for the current [StructType]
    ///
    /// Example: `UpdateTodos`
    pub fn prefix(&self) -> &'static str {
        match self {
            StructType::Read => "",
            StructType::Update => "Update",
            StructType::Create => "Create",
        }
    }

    /// Get the suffix for the current [StructType]
    ///
    /// Example: `TodosForm`
    pub fn suffix(&self) -> &'static str {
        match self {
            StructType::Read => "",
            StructType::Update => "",
            StructType::Create => "",
        }
    }

    /// Format a struct with all prefix- and suffixes
    ///
    /// Example: `UpdateTodos`
    pub fn format(&self, name: &str) -> String {
        format!(
            "{struct_prefix}{struct_name}{struct_suffix}",
            struct_prefix = self.prefix(),
            struct_name = name,
            struct_suffix = self.suffix()
        )
    }
}

#[derive(Debug)]
struct Struct<'a> {
    /// Struct name (like `UpdateTodos`)
    identifier: String,
    /// Type of the Struct
    ty: StructType,
    /// Parsed table reference
    table: &'a ParsedTableMacro,
    /// Generation options specific for the current table
    opts: TableOptions<'a>,
    /// Global generation options
    config: &'a GenerationConfig<'a>,
    /// Storage for the finished generated code
    rendered_code: Option<String>,
    /// Cache for if this struct even has any fields
    has_fields: Option<bool>, // note: this is only correctly set after a call to render() which gets called in Struct::new()
}

#[derive(Debug, Clone)]
pub struct StructField {
    /// Name for the field
    // TODO: should this be a Ident instead of a string?
    pub name: String,
    /// Base Rust type, like "String" or "i32" or "u32"
    pub base_type: String,
    /// Indicate that this field is optional
    pub is_optional: bool,
}

impl StructField {
    /// Assemble the current options into a rust type, like `base_type: String, is_optional: true` to `Option<String>`
    pub fn to_rust_type(&self) -> Cow<str> {
        if self.is_optional {
            return format!("Option<{}>", self.base_type).into();
        }

        return self.base_type.as_str().into();
    }
}

impl From<&ParsedColumnMacro> for StructField {
    fn from(value: &ParsedColumnMacro) -> Self {
        let name = value.name.to_string();
        // convert integers to proper rust integers
        let base_type = if value.is_unsigned {
            value.ty.replace('i', "u")
        } else {
            value.ty.clone()
        };

        Self {
            name,
            base_type,
            is_optional: value.is_nullable,
        }
    }
}

/// Collection of all dervies available
pub mod derives {
    pub const DEBUG: &str = "Debug";
    pub const DEFAULT: &str = "Default";
    pub const CLONE: &str = "Clone";
    pub const QUERYABLE: &str = "Queryable";
    pub const INSERTABLE: &str = "Insertable";
    pub const SERIALIZE: &str = "Serialize";
    pub const DESERIALIZE: &str = "Deserialize";
    pub const ASCHANGESET: &str = "AsChangeset";
    pub const SELECTABLE: &str = "Selectable";
    pub const IDENTIFIABLE: &str = "Identifiable";
    pub const ASSOCIATIONS: &str = "Associations";
}

impl<'a> Struct<'a> {
    /// Create a new instance
    pub fn new(
        ty: StructType,
        table: &'a ParsedTableMacro,
        config: &'a GenerationConfig<'_>,
    ) -> Self {
        let mut obj = Self {
            identifier: ty.format(table.struct_name.as_str()),
            opts: config.table(&table.name.to_string()),
            table,
            ty,
            config,
            rendered_code: None,
            has_fields: None,
        };
        obj.render();
        obj
    }

    pub fn has_code(&self) -> bool {
        self.rendered_code.is_some()
    }

    /// Get the rendered code, or a empty string
    pub fn code(&self) -> &str {
        self.rendered_code.as_deref().unwrap_or_default()
    }

    /// Get if the current struct has fields
    ///
    /// Currently panics if [`render`](#render) has not been called yet
    pub fn has_fields(&self) -> bool {
        self.has_fields.unwrap()
    }

    fn attr_tsync(&self) -> &'static str {
        #[cfg(feature = "tsync")]
        match self.opts.get_tsync() {
            true => "#[tsync::tsync]\n",
            false => "",
        }
        #[cfg(not(feature = "tsync"))]
        ""
    }

    /// Assemble all derives for the struct
    fn attr_derive(&self) -> String {
        let mut derives_vec = Vec::with_capacity(10);
        // Default derives that exist on every struct
        derives_vec.extend_from_slice(&[derives::DEBUG, derives::CLONE]);

        if self.config.table(&self.table.name.to_string()).get_serde() {
            derives_vec.extend_from_slice(&[derives::SERIALIZE, derives::DESERIALIZE]);
        }

        match self.ty {
            StructType::Read => {
                // derives that always exist, regardless of extra conditions
                derives_vec.extend_from_slice(&[derives::QUERYABLE, derives::SELECTABLE]);

                if !self.table.foreign_keys.is_empty() {
                    derives_vec.extend_from_slice(&[derives::ASSOCIATIONS, derives::IDENTIFIABLE]);
                }
            }
            StructType::Update => {
                // NOTE: the following might not be fully necessary and there is not test for this, see https://github.com/Wulf/dsync/pull/87/files/4ca7054981d6925c3709643e3020c31666024ce2#r1375325415 for a explanation
                if !self
                    .fields()
                    .iter()
                    .all(|f| self.table.primary_key_column_names().contains(&f.name))
                {
                    derives_vec.push(derives::ASCHANGESET);
                }

                derives_vec.push(derives::DEFAULT);
            }
            StructType::Create => derives_vec.extend_from_slice(&[derives::INSERTABLE]),
        }

        format!("#[derive({})]", derives_vec.join(", "))
    }

    /// Convert [ParsedColumnMacro]'s to [StructField]'s
    ///
    /// Fields filtered out:
    /// - in Create-Structs: auto-generated fields
    /// - in Update-Structs: the primary key(s)
    fn fields(&self) -> Vec<StructField> {
        self.table
            .columns
            .iter()
            .filter(|c| {
                let is_autogenerated = self
                    .opts
                    .autogenerated_columns
                    .as_deref()
                    .unwrap_or_default()
                    .contains(&c.name.to_string().as_str());

                match self.ty {
                    StructType::Read => true,
                    StructType::Update => {
                        let is_pk = self.table.primary_key_columns.contains(&c.name);

                        !is_pk
                    }
                    StructType::Create => !is_autogenerated,
                }
            })
            .map(StructField::from)
            .collect()
    }

    /// Render the full struct
    fn render(&mut self) {
        let ty = self.ty;
        let table = &self.table;

        if self.opts.get_readonly() {
            match ty {
                StructType::Read => (),
                StructType::Update | StructType::Create => {
                    self.has_fields = Some(false);
                    self.rendered_code = None;

                    return;
                }
            }
        }

        let primary_keys: Vec<String> = table.primary_key_column_names();

        let belongs_to = table
            .foreign_keys
            .iter()
            .map(|fk| {
                format!(
                    ", belongs_to({foreign_table_name}, foreign_key={join_column})",
                    foreign_table_name = fk.0.to_string().to_pascal_case(),
                    join_column = fk.1
                )
            })
            .collect::<Vec<String>>()
            .join(" ");

        let fields = self.fields();

        if fields.is_empty() {
            self.has_fields = Some(false);
            self.rendered_code = None;
            return;
        }

        let lifetimes = {
            let lifetimes = match self.ty {
                StructType::Read => "",
                StructType::Update => self.opts.get_update_str_type().get_lifetime(),
                StructType::Create => self.opts.get_create_str_type().get_lifetime(),
            };

            if lifetimes.is_empty() {
                String::new()
            } else {
                format!("<{}>", lifetimes)
            }
        };

        let mut lines = Vec::with_capacity(fields.len());
        for mut f in fields.into_iter() {
            let field_name = &f.name;

            if f.base_type == "String" {
                f.base_type = match self.ty {
                    StructType::Read => f.base_type,
                    StructType::Update => self.opts.get_update_str_type().as_str().to_string(),
                    StructType::Create => self.opts.get_create_str_type().as_str().to_string(),
                }
            }

            let mut field_type = f.to_rust_type();

            // always wrap a field in "Option<>" for a update struct, instead of flat options
            // because otherwise you could not differentiate between "Dont touch this field" and "Set field to null"
            // also see https://github.com/Wulf/dsync/pull/83#issuecomment-1741905691
            if self.ty == StructType::Update {
                field_type = format!("Option<{}>", field_type).into();
            }

            lines.push(format!(r#"    pub {field_name}: {field_type},"#));
        }

        let struct_code = formatdoc!(
            r#"
            {tsync_attr}{derive_attr}
            #[diesel(table_name={table_name}{primary_key}{belongs_to})]
            pub struct {struct_name}{lifetimes} {{
            {lines}
            }}
            "#,
            tsync_attr = self.attr_tsync(),
            derive_attr = self.attr_derive(),
            table_name = table.name,
            struct_name = ty.format(&table.struct_name),
            lifetimes = lifetimes,
            primary_key = if ty != StructType::Read {
                "".to_string()
            } else {
                format!(", primary_key({})", primary_keys.join(","))
            },
            belongs_to = if ty != StructType::Read {
                "".to_string()
            } else {
                belongs_to
            },
            lines = lines.join("\n"),
        );

        self.has_fields = Some(true);
        self.rendered_code = Some(struct_code);
    }
}

/// Generate all functions (insides of the `impl StuctName { here }`)
fn build_table_fns(
    table: &ParsedTableMacro,
    config: &GenerationConfig,
    create_struct: Struct,
    update_struct: Struct,
) -> String {
    let table_options = config.table(&table.name.to_string());

    let primary_column_name_and_type: Vec<(String, String)> = table
        .primary_key_columns
        .iter()
        .map(|pk| {
            let col = table
                .columns
                .iter()
                .find(|it| it.name.to_string().eq(pk.to_string().as_str()))
                .expect("Primary key column doesn't exist in table");

            (col.name.to_string(), col.ty.to_string())
        })
        .collect();

    let item_id_params = primary_column_name_and_type
        .iter()
        .map(|name_and_type| {
            format!(
                "param_{name}: {ty}",
                name = name_and_type.0,
                ty = name_and_type.1
            )
        })
        .collect::<Vec<String>>()
        .join(", ");
    let item_id_filters = primary_column_name_and_type
        .iter()
        .map(|name_and_type| {
            format!(
                "filter({name}.eq(param_{name}))",
                name = name_and_type.0.to_string()
            )
        })
        .collect::<Vec<String>>()
        .join(".");

    // template variables
    let table_name = table.name.to_string();
    #[cfg(feature = "async")]
    let async_keyword = if table_options.get_async() {
        " async"
    } else {
        ""
    };
    #[cfg(not(feature = "async"))]
    let async_keyword = "";
    #[cfg(feature = "async")]
    let await_keyword = if table_options.get_async() {
        ".await"
    } else {
        ""
    };
    #[cfg(not(feature = "async"))]
    let await_keyword = "";
    let struct_name = &table.struct_name;
    let schema_path = &config.schema_path;
    let create_struct_identifier = &create_struct.identifier;
    let update_struct_identifier = &update_struct.identifier;
    let is_readonly = table_options.get_readonly();

    let mut buffer = String::new();

    if !config.once_common_structs {
        buffer.push_str(&generate_common_structs(&table_options));
        buffer.push('\n');
    }

    buffer.push_str(&format!("impl {struct_name} {{"));

    if !is_readonly {
        if create_struct.has_fields() {
            buffer.push_str(&format!(
            r##"
    pub{async_keyword} fn create(db: &mut ConnectionType, item: &{create_struct_identifier}) -> QueryResult<Self> {{
        use {schema_path}{table_name}::dsl::*;

        insert_into({table_name}).values(item).get_result::<Self>(db){await_keyword}
    }}
"##
        ));
        } else {
            buffer.push_str(&format!(
                r##"
    pub{async_keyword} fn create(db: &mut ConnectionType) -> QueryResult<Self> {{
        use {schema_path}{table_name}::dsl::*;

        insert_into({table_name}).default_values().get_result::<Self>(db){await_keyword}
    }}
"##
            ));
        }
    }

    buffer.push_str(&format!(
        r##"
    pub{async_keyword} fn read(db: &mut ConnectionType, {item_id_params}) -> QueryResult<Self> {{
        use {schema_path}{table_name}::dsl::*;

        {table_name}.{item_id_filters}.first::<Self>(db){await_keyword}
    }}
"##
    ));

    buffer.push_str(&format!(r##"
    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub{async_keyword} fn paginate(db: &mut ConnectionType, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {{
        use {schema_path}{table_name}::dsl::*;

        let page_size = if page_size < 1 {{ 1 }} else {{ page_size }};
        let total_items = {table_name}.count().get_result(db){await_keyword}?;
        let items = {table_name}.limit(page_size).offset(page * page_size).load::<Self>(db){await_keyword}?;

        Ok(PaginationResult {{
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        }})
    }}
"##));

    // TODO: If primary key columns are attached to the form struct (not optionally)
    // then don't require item_id_params (otherwise it'll be duplicated)

    // if has_update_struct {
    if update_struct.has_fields() && !is_readonly {
        // It's possible we have a form struct with all primary keys (for example, for a join table).
        // In this scenario, we also have to check whether there are any updatable columns for which
        // we should generate an update() method.

        buffer.push_str(&format!(r##"
    pub{async_keyword} fn update(db: &mut ConnectionType, {item_id_params}, item: &{update_struct_identifier}) -> QueryResult<Self> {{
        use {schema_path}{table_name}::dsl::*;

        diesel::update({table_name}.{item_id_filters}).set(item).get_result(db){await_keyword}
    }}
"##));
    }

    if !is_readonly {
        buffer.push_str(&format!(
            r##"
    pub{async_keyword} fn delete(db: &mut ConnectionType, {item_id_params}) -> QueryResult<usize> {{
        use {schema_path}{table_name}::dsl::*;

        diesel::delete({table_name}.{item_id_filters}).execute(db){await_keyword}
    }}
"##
        ));
    }

    buffer.push('}');

    buffer
}

/// Generate common structs
pub fn generate_common_structs(table_options: &TableOptions<'_>) -> String {
    #[cfg(feature = "tsync")]
    let tsync = match table_options.get_tsync() {
        true => "#[tsync::tsync]",
        false => "",
    };
    #[cfg(not(feature = "tsync"))]
    let tsync = "";

    formatdoc!(
        r##"{tsync}
        #[derive({debug_derive}, {serde_derive})]
        pub struct PaginationResult<T> {{
            pub items: Vec<T>,
            pub total_items: i64,
            /// 0-based index
            pub page: i64,
            pub page_size: i64,
            pub num_pages: i64,
        }}
        "##,
        serde_derive = if table_options.get_serde() {
            derives::SERIALIZE
        } else {
            ""
        },
        debug_derive = derives::DEBUG
    )
}

/// Generate connection-type type
pub fn generate_connection_type(config: &GenerationConfig) -> String {
    format!(
        "\ntype ConnectionType = {connection_type};",
        connection_type = config.connection_type,
    )
}

/// Generate all imports for the struct file that are required
fn build_imports(table: &ParsedTableMacro, config: &GenerationConfig) -> String {
    // Note: i guess this could also just be a string that is appended to, or a vec of "Cow", but i personally think this is the most use-able
    // because you dont have to think of any context style (like forgetting to put "\n" before / after something)
    let mut imports_vec = Vec::with_capacity(10);
    imports_vec.push("use crate::diesel::*;".into());

    let table_options = config.table(&table.name.to_string());
    imports_vec.extend(table.foreign_keys.iter().map(|fk| {
        format!(
            "use {model_path}{foreign_table_name_model}::{singular_struct_name};",
            foreign_table_name_model = fk.0.to_string().to_snake_case().to_lowercase(),
            singular_struct_name = fk.0.to_string().to_pascal_case(),
            model_path = config.model_path
        )
    }));
    #[cfg(feature = "async")]
    if table_options.get_async() {
        imports_vec.push("use diesel_async::RunQueryDsl;".into());
    }

    // no "::" because that is already included in the schema_path
    imports_vec.push(format!("use {}*;", config.schema_path));

    if table_options.get_serde() {
        imports_vec.push("use serde::{Deserialize, Serialize};".into());
    };

    if table_options.get_fns() {
        imports_vec.push("use diesel::QueryResult;".into());
    };

    if config.once_common_structs || config.once_connection_type {
        imports_vec.push(format!("use {}common::*;", config.model_path));
    };

    // this needs to be last, because it not really is a import, so it would split the import sections
    if table_options.get_fns() && !config.once_connection_type {
        imports_vec.push(generate_connection_type(config));
    };

    imports_vec.join("\n")
}

/// Generate a full file for a given diesel table
pub fn generate_for_table(table: &ParsedTableMacro, config: &GenerationConfig) -> String {
    let table_options = config.table(&table.name.to_string());
    // first, we generate struct code
    let read_struct = Struct::new(StructType::Read, table, config);
    let update_struct = Struct::new(StructType::Update, table, config);
    let create_struct = Struct::new(StructType::Create, table, config);

    let mut structs = String::from(read_struct.code());
    if create_struct.has_code() {
        structs.push('\n');
        structs.push_str(create_struct.code());
    }

    if update_struct.has_code() {
        structs.push('\n');
        structs.push_str(update_struct.code());
    }

    let functions = if table_options.get_fns() {
        build_table_fns(table, config, create_struct, update_struct)
    } else {
        "".to_string()
    };
    let imports = build_imports(table, config);

    format!("{FILE_SIGNATURE}\n\n{imports}\n\n{structs}\n{functions}\n")
}
