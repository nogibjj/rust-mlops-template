/* Life-expectancy CLI that uses Polars and Clap
*/

use clap::Parser;
use dscli::CSV_FILE;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "A data science CLI that uses Polars and Clap",
    after_help = "Example: cargo run -- print --rows 3"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Print {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
        #[clap(long, default_value = "10")]
        rows: usize,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Print { path, rows }) => {
            let df = dscli::read_csv(&path);
            dscli::print_df(&df, rows);
        }
        None => println!("No command given"),
    }
}
