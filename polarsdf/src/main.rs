//command-line tool that reads a CSV file and prints the contents of the file as a DataFrame
use clap::Parser;
use polars::prelude::*;

const CSV_FILE: &str = "src/data/global-life-expt-2022.csv";

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "A command-line tool that reads a CSV file and prints the contents of the file as a DataFrame",
    after_help = "Example: polarsdf --file data/global-life-expt-2022.csv"
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
    },
    Describe {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
    },
}
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Print { path }) => {
            let df = CsvReader::from_path(path).unwrap().finish().unwrap();
            println!("{:?}", df);
        }
        Some(Commands::Describe { path }) => {
            let df = CsvReader::from_path(path).unwrap().finish().unwrap();
            println!("{:?}", df);
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
