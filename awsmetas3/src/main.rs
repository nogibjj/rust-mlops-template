/*A Command-line tool to Interrogate AWS S3.
Determines information about AWS S3 buckets and objects.
*/
use clap::Parser;
use humansize::{format_size, DECIMAL};

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "Finds out information about AWS S3",
    after_help = "Example: awsmetas3 account-size"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Buckets {},
    AccountSize {
        #[clap(short, long)]
        verbose: Option<bool>,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let client = awsmetas3::client().await.unwrap();
    match args.command {
        Some(Commands::Buckets {}) => {
            let buckets = awsmetas3::list_buckets(&client).await.unwrap();
            //print count of buckets
            println!("Found {} buckets", buckets.len());
            println!("Buckets: {:?}", buckets);
        }
        /*print total size of all buckets in human readable format
        Use list_bucket_sizes to get a list of all buckets in an AWS S3 account
        */
        Some(Commands::AccountSize { verbose }) => {
            let bucket_sizes = awsmetas3::list_bucket_sizes(&client, verbose)
                .await
                .unwrap();
            let total_size: i64 = bucket_sizes.iter().sum();
            println!(
                "Total size of all buckets is {}",
                format_size(total_size as u64, DECIMAL)
            );
        }
        None => println!("No command specified"),
    }
}
