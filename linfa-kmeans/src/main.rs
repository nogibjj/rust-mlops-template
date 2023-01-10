/* run kmeans */
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Noah Gift")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    Cluster {},
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Cluster {}) => {
            println!("Running KMeans");
            linfa_kmeans::run();
        }
        None => {
            println!("No command given");
        }
    }
}
