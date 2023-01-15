/*Command-line to for lasso regression that accepts training ratio */
use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "A command-line tool that performs lasso regression",
    after_help = "Example: "
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Train {
        #[clap(short, long, default_value = "0.8")]
        ratio: f32,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Train { ratio }) => {
            println!("Training ratio: {}", ratio);
            let _status = regression_cli::predict(ratio);
        }
        None => {
            println!("No command given");
        }
    }
}
