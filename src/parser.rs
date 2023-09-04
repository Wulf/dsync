use inflector::Inflector;
use syn::Ident;
use syn::Item::Macro;

use crate::{code, Error, GenerationConfig, Result};

pub const FILE_SIGNATURE: &str = "/* @generated and managed by dsync */";

// TODO: handle postgres array types
// TODO: handle postgres tuple/record types

#[derive(Debug, Clone)]
pub struct ParsedColumnMacro {
    /// Rust type to use (like `String`)
    pub ty: String,
    /// Rust ident for the field name
    pub name: Ident,
    pub is_nullable: bool,
    pub is_unsigned: bool,
}

/// Struct to hold all information needed from a parsed `diesel::table!` macro
#[derive(Debug, Clone)]
pub struct ParsedTableMacro {
    /// Name of the table
    pub name: Ident,
    /// Rust struct name to use
    pub struct_name: String,
    /// All parsed columns
    pub columns: Vec<ParsedColumnMacro>,
    /// All Primary key column idents
    pub primary_key_columns: Vec<Ident>,
    /// All foreign key relations (foreign_table_name, local_join_column)
    pub foreign_keys: Vec<(
        ForeignTableName,
        JoinColumn, /* this is the column from this table which maps to the foreign table's primary key*/
    )>,
    /// Final generated code
    pub generated_code: String,
}

impl ParsedTableMacro {
    pub fn primary_key_column_names(&self) -> Vec<String> {
        self.primary_key_columns
            .iter()
            .map(|i| i.to_string())
            .collect()
    }
}

type ForeignTableName = Ident;
type JoinColumn = String;

/// Parsed representation of the `diesel::joinable!` macro
#[derive(Debug, Clone)]
pub struct ParsedJoinMacro {
    /// Table ident with the foreign key
    pub table1: Ident,
    /// Table ident for the foreign key
    pub table2: Ident,
    /// Column ident of for the foreign key
    pub table1_columns: String,
}

/// Try to parse a diesel schema file contents
pub fn parse_and_generate_code(
    schema_file_contents: String,
    config: &GenerationConfig,
) -> Result<Vec<ParsedTableMacro>> {
    let schema_file = syn::parse_file(&schema_file_contents).unwrap();

    let mut tables: Vec<ParsedTableMacro> = vec![];

    for item in schema_file.items {
        if let Macro(macro_item) = item {
            let macro_identifier = macro_item
                .mac
                .path
                .segments
                .last()
                .ok_or(Error::other("could not read identifier for macro"))?
                .ident
                .to_string();

            match macro_identifier.as_str() {
                "table" => {
                    let parsed_table = handle_table_macro(macro_item, config)?;

                    // make sure the table isn't ignored
                    let table_options = config.table(parsed_table.name.to_string().as_str());
                    if !table_options.get_ignore() {
                        tables.push(parsed_table);
                    }
                }
                "joinable" => {
                    let parsed_join = handle_joinable_macro(macro_item)?;

                    for table in tables.iter_mut() {
                        if parsed_join
                            .table1
                            .to_string()
                            .eq(table.name.to_string().as_str())
                        {
                            table.foreign_keys.push((
                                parsed_join.table2.clone(),
                                parsed_join.table1_columns.clone(),
                            ));
                            break;
                        }
                    }
                }
                _ => {}
            };
        }
    }

    for table in tables.iter_mut() {
        table.generated_code = code::generate_for_table(table.clone(), config);
    }

    Ok(tables)
}

fn handle_joinable_macro(macro_item: syn::ItemMacro) -> Result<ParsedJoinMacro> {
    // println!("joinable! macro: {:#?}", macro_item);

    let mut table1_name: Option<Ident> = None;
    let mut table2_name: Option<Ident> = None;
    let mut table2_join_column: Option<String> = None;

    for item in macro_item.mac.tokens.into_iter() {
        match &item {
            proc_macro2::TokenTree::Ident(ident) => {
                if table1_name.is_none() {
                    table1_name = Some(ident.clone());
                } else if table2_name.is_none() {
                    table2_name = Some(ident.clone());
                }
            }
            proc_macro2::TokenTree::Group(group) => {
                if table1_name.is_none() || table2_name.is_none() {
                    return Err(Error::unsupported_schema_format(
                        "encountered join column group too early",
                    ));
                } else {
                    table2_join_column = Some(group.stream().to_string());
                }
            }
            _ => {}
        }
    }

    Ok(ParsedJoinMacro {
        table1: table1_name.ok_or(Error::unsupported_schema_format(
            "could not determine first join table name",
        ))?,
        table2: table2_name.ok_or(Error::unsupported_schema_format(
            "could not determine second join table name",
        ))?,
        table1_columns: table2_join_column.ok_or(Error::unsupported_schema_format(
            "could not determine join column name",
        ))?,
    })
}

/// Try to parse a `diesel::table!` macro
fn handle_table_macro(
    macro_item: syn::ItemMacro,
    config: &GenerationConfig,
) -> Result<ParsedTableMacro> {
    let mut table_name_ident: Option<Ident> = None;
    let mut table_primary_key_idents: Vec<Ident> = vec![];
    let mut table_columns: Vec<ParsedColumnMacro> = vec![];

    let mut skip_until_semicolon = false;
    let mut skip_square_brackets = false;

    for item in macro_item.mac.tokens.into_iter() {
        if skip_until_semicolon {
            if let proc_macro2::TokenTree::Punct(punct) = item {
                if punct.as_char() == ';' {
                    skip_until_semicolon = false;
                }
            }
            continue;
        }

        match &item {
            proc_macro2::TokenTree::Punct(punct) => {
                // skip any "#[]"
                match punct.to_string().as_str() {
                    "#" => {
                        skip_square_brackets = true;
                        continue;
                    }
                    _ => {}
                }
            }
            proc_macro2::TokenTree::Ident(ident) => {
                // skip any "use" statements
                if ident.to_string().eq("use") {
                    skip_until_semicolon = true;
                    continue;
                }

                table_name_ident = Some(ident.clone());
            }
            proc_macro2::TokenTree::Group(group) => {
                if skip_square_brackets {
                    if group.delimiter() == proc_macro2::Delimiter::Bracket {
                        skip_square_brackets = false;
                    }
                    continue;
                }

                if group.delimiter() == proc_macro2::Delimiter::Parenthesis {
                    // primary keys group
                    // println!("GROUP-keys {:#?}", group);
                    for key_token in group.stream().into_iter() {
                        if let proc_macro2::TokenTree::Ident(ident) = key_token {
                            table_primary_key_idents.push(ident)
                        }
                    }
                } else if group.delimiter() == proc_macro2::Delimiter::Brace {
                    // columns group
                    // println!("GROUP-cols {:#?}", group);

                    let mut column_name: Option<Ident> = None;
                    let mut column_type: Option<Ident> = None;
                    let mut column_nullable: bool = false;
                    let mut column_unsigned: bool = false;

                    for column_tokens in group.stream().into_iter() {
                        match column_tokens {
                            proc_macro2::TokenTree::Group(_) => {
                                continue;
                            }
                            proc_macro2::TokenTree::Ident(ident) => {
                                if column_name.is_none() {
                                    column_name = Some(ident.clone());
                                } else if ident.to_string().eq_ignore_ascii_case("Nullable") {
                                    column_nullable = true;
                                } else if ident.to_string().eq_ignore_ascii_case("Unsigned") {
                                    column_unsigned = true;
                                } else {
                                    column_type = Some(ident.clone());
                                }
                            }
                            proc_macro2::TokenTree::Punct(punct) => {
                                let char = punct.as_char();

                                if char == '#' || char == '-' || char == '>' {
                                    // nothing for arrow or any additional #[]
                                    continue;
                                } else if char == ','
                                    && column_name.is_some()
                                    && column_type.is_some()
                                {
                                    // end of column def!

                                    // add the column
                                    table_columns.push(ParsedColumnMacro {
                                        name: column_name.ok_or(
                                            Error::unsupported_schema_format(
                                                "Invalid column name syntax",
                                            ),
                                        )?,
                                        ty: schema_type_to_rust_type(
                                            column_type
                                                .ok_or(Error::unsupported_schema_format(
                                                    "Invalid column type syntax",
                                                ))?
                                                .to_string(),
                                            config,
                                        )?,
                                        is_nullable: column_nullable,
                                        is_unsigned: column_unsigned,
                                    });

                                    // reset the properties
                                    column_name = None;
                                    column_type = None;
                                    column_unsigned = false;
                                    column_nullable = false;
                                }
                            }
                            _ => {
                                return Err(Error::unsupported_schema_format(
                                    "Invalid column definition token in diesel table macro",
                                ))
                            }
                        }
                    }

                    if column_name.is_some()
                        || column_type.is_some()
                        || column_nullable
                        || column_unsigned
                    {
                        // looks like a column was in the middle of being parsed, let's panic!
                        return Err(Error::unsupported_schema_format(
                            "It seems a column was partially defined",
                        ));
                    }
                } else {
                    return Err(Error::unsupported_schema_format(
                        "Invalid delimiter in diesel table macro group",
                    ));
                }
            }
            _ => {
                return Err(Error::unsupported_schema_format(
                    "Invalid token tree item in diesel table macro",
                ))
            }
        }
    }

    Ok(ParsedTableMacro {
        name: table_name_ident
            .clone()
            .ok_or(Error::unsupported_schema_format(
                "Could not extract table name from schema file",
            ))?,
        struct_name: table_name_ident
            .unwrap()
            .to_string()
            .to_pascal_case()
            .to_singular(),
        columns: table_columns,
        primary_key_columns: table_primary_key_idents,
        foreign_keys: vec![],
        generated_code: format!(
            "{FILE_SIGNATURE}\n\nFATAL ERROR: nothing was generated; this shouldn't be possible."
        ),
    })
}

// A function to translate diesel schema types into rust types
//
// reference: https://github.com/diesel-rs/diesel/blob/master/diesel/src/sql_types/mod.rs
// exact reference; https://github.com/diesel-rs/diesel/blob/292ac5c0ed6474f96734ba2e99b95b442064f69c/diesel/src/mysql/types/mod.rs
//
// The docs page for sql_types is comprehensive but it hides some alias types like Int4, Float8, etc.:
// https://docs.rs/diesel/latest/diesel/sql_types/index.html
fn schema_type_to_rust_type(schema_type: String, config: &GenerationConfig) -> Result<String> {
    Ok(match schema_type.to_lowercase().as_str() {
        "unsigned" => return Err(Error::unsupported_type("Unsigned types are not yet supported, please open an issue if you need this feature!")), // TODO: deal with this later
        "inet" => return Err(Error::unsupported_type("Unsigned types are not yet supported, please open an issue if you need this feature!")), // TODO: deal with this later
        "cidr" => return Err(Error::unsupported_type("Unsigned types are not yet supported, please open an issue if you need this feature!")), // TODO: deal with this later

        // boolean
        "bool" => "bool",

        // numbers
        "tinyint" => "i8",
        "smallint" => "i16",
        "smallserial" => "i16",
        "int2" => "i16",
        "int4" => "i32",
        "int4range" => "(std::collections::Bound<i32>, std::collections::Bound<i32>)",
        "integer" => "i32",
        "serial" => "i32",
        "bigint" => "i64",
        "bigserial" => "i64",
        "int8" => "i64",
        "int8range" => "(std::collections::Bound<i64>, std::collections::Bound<i64>)",
        "float" => "f32",
        "float4" => "f32",
        "double" => "f64",
        "float8" => "f64",
        "numeric" => "bigdecimal::BigDecimal",
        "numrange" => "(std::collections::Bound<bigdecimal::BigDecimal>, std::collections::Bound<bigdecimal::BigDecimal>)",
        "decimal" => "bigdecimal::BigDecimal",

        // string
        "text" => "String",
        "varchar" => "String",
        "bpchar" => "String",
        "char" => "String",
        "tinytext" => "String",
        "mediumtext" => "String",
        "longtext" => "String",

        // bytes
        "binary" => "Vec<u8>",
        "bytea" => "Vec<u8>",
        "tinyblob" => "Vec<u8>",
        "blob" => "Vec<u8>",
        "mediumblob" => "Vec<u8>",
        "longblob" => "Vec<u8>",
        "varbinary" => "Vec<u8>",
        "bit" => "Vec<u8>",

        // date & time
        "date" => "chrono::NaiveDate",
        "daterange" => "(std::collections::Bound<chrono::NaiveDate>, std::collections::Bound<chrono::NaiveDate>)",
        "datetime" => "chrono::NaiveDateTime",
        "time" => "chrono::NaiveTime",
        "timestamp" => "chrono::NaiveDateTime",
        "tsrange" => "(std::collections::Bound<chrono::NaiveDateTime>, std::collections::Bound<chrono::NaiveDateTime>)",
        "timestamptz" => "chrono::DateTime<chrono::Utc>",
        "timestamptzsqlite" => "chrono::DateTime<chrono::Utc>",
        "tstzrange" => "(std::collections::Bound<chrono::DateTime<chrono::Utc>>, std::collections::Bound<chrono::DateTime<chrono::Utc>>)",

        // json
        "json" => "serde::Value",
        "jsonb" => "serde_json::Value",

        // misc
        "uuid" => "uuid::Uuid",
        "interval" => "PgInterval",
        "oid" => "u32",
        "money" => "PgMoney",
        "macaddr" => "[u8; 6]",
        // "inet" => "either ipnetwork::IpNetwork or ipnet::IpNet (TODO)",
        // "cidr" => "either ipnetwork::IpNetwork or ipnet::IpNet (TODO)",

        /*
            // panic if no type is found (this means generation is broken for this particular schema)
            _ => panic!("Unknown type found '{schema_type}', please report this!")
         */
        _ => {
            let schema_path = &config.schema_path;
            // return the schema type if no type is found (this means generation is broken for this particular schema)
            let _type = format!("{schema_path}sql_types::{schema_type}");
            return Ok(_type);
        }
    }.to_string())
}
