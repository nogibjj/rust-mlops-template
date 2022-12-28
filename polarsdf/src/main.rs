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
    Describe {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
    },
    Schema {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
    },
    Shape {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
    },
    Sort {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
        #[clap(long, default_value = "2020")]
        year: String,
        #[clap(long, default_value = "10")]
        rows: usize,
        #[clap(long, default_value = "true")]
        order: bool,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Print { path, rows }) => {
            let df = polarsdf::read_csv(&path);
            println!("{:?}", df.head(Some(rows)));
        }
        Some(Commands::Describe { path }) => {
            let df = polarsdf::read_csv(&path);
            println!("{:?}", df);
        }
        Some(Commands::Schema { path }) => {
            let df = polarsdf::read_csv(&path);
            println!("{:?}", df.schema());
        }
        Some(Commands::Shape { path }) => {
            let df = polarsdf::read_csv(&path);
            println!("{:?}", df.shape());
        }
        Some(Commands::Sort {
            path,
            year,
            rows,
            order,
        }) => {
            let df = polarsdf::read_csv(&path);
            let country_column_name = "Country Name";
            //select the country column and the year string passed in and return a new dataframe
            let vs = df.select_series([country_column_name, &year]).unwrap();
            //convert the Vec<Series> to a DataFrame
            let df2 = DataFrame::new(vs).unwrap();
            //drop any rows with null values and return a new dataframe
            let df2: DataFrame = df2.drop_nulls(None).unwrap();
            //sort the dataframe by the year column and by order passed in
            let df2 = df2.sort([&year], order).unwrap();

            //print the first "rows" of the dataframe
            println!("{:?}", df2.head(Some(rows)));
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
