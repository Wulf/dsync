use dsync::{GenerationConfig, TableOptions};
use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;

const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[allow(dead_code)]
#[derive(Debug, StructOpt, Clone)]
#[structopt(about = DESCRIPTION)]
struct Args {
    /// Input file
    #[structopt(
        short = "i",
        long = "input",
        help = "Required; rust file to read diesel schema information from",
        required = true
    )]
    input: PathBuf,

    /// Output file, stdout if not present
    #[structopt(
        parse(from_os_str),
        short = "o",
        long = "output",
        help = "Required; directory to write generated code to"
    )]
    output: PathBuf,

    #[cfg(feature = "tsync")]
    #[structopt(
        long = "tsync",
        help = "Optional: adds the #[tsync] attribute to all structs; see https://github.com/Wulf/tsync"
    )]
    tsync: bool,

    #[structopt(
        short = "g",
        long = "autogenerated-columns",
        help = "Optional; List of columns which are automatically generated but are not primary keys (for example: `created_at`, `updated_at`, etc.)"
    )]
    autogenerated_columns: Option<Vec<String>>,

    #[structopt(
        short = "c",
        long = "connection-type",
        help = "Required: rust type which describes a connection, for example: `diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>`"
    )]
    connection_type: String,
}

fn main() {
    let args: Args = Args::from_args();
    let cols = args.autogenerated_columns.unwrap_or_default();
    let mut default_table_options = TableOptions::default().autogenerated_columns(
            cols
            .iter()
            .map(|t| t.as_str())
            .collect::<Vec<&str>>(),
    );

    #[cfg(feature = "tsync")]
    if args.tsync {
        default_table_options = default_table_options.tsync();
    }


    dsync::generate_files(
        args.input,
        args.output,
        GenerationConfig {
            default_table_options,
            table_options: HashMap::from([]),
            connection_type: args.connection_type,
        },
    );
}
