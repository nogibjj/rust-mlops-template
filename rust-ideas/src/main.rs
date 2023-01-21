//Command-line tool to get rust project ideas
use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "A command-line tool that recommends a popular Rust crate to the user.",
    after_help = "Example: cargo run -- popular"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(about = "Get a popular Rust crate")]
    Popular {
        #[clap(short, long, default_value = "10")]
        number: Option<usize>,
    },
    Random {},
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Popular { number }) => {
            let popular_crates = rust_ideas::get_popular_crates(number.unwrap()).unwrap();
            println!("Popular crates: {:?}", popular_crates);
        }
        Some(Commands::Random {}) => {
            let random_crate = rust_ideas::get_random_crate().unwrap();
            println!("Random crate: {:?}", random_crate);
        }
        None => {
            println!("No command given");
        }
    }
}
