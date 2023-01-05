//Searches a path for duplicate files
//Compares serial and parallel versions of the program
use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "Compares serial and parallel versions of the program",
    after_help = "Example: parallel parallel --path /src/data/"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Parallel {
        #[clap(short, long, default_value = "src/data/")]
        path: String,
    },
    Serial {
        #[clap(short, long, default_value = "src/data/")]
        path: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Parallel { path }) => {
            println!("Parallel version of the program");
            let files = parallel::walk(&path).unwrap();
            let checksums = parallel::checksum_par(files).unwrap();
            for (checksum, files) in checksums {
                if files.len() > 1 {
                    println!("{}:", checksum);
                    for file in files {
                        println!("\t{}", file);
                    }
                }
            }
        }
        Some(Commands::Serial { path }) => {
            println!("Serial version of the program");
            let files = parallel::walk(&path).unwrap();
            let checksums = parallel::checksum(files).unwrap();
            for (checksum, files) in checksums {
                if files.len() > 1 {
                    println!("{}:", checksum);
                    for file in files {
                        println!("\t{}", file);
                    }
                }
            }
        }
        None => println!("No command specified"),
    }
}
