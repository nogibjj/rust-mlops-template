/*
A MMOG game server written in Rust that let's people play the casino game roulette
with their friends.  The game is played from the command-line and the server.
 */
use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "Client Server Roulette Game",
    after_help = "For more information please visit: https://github.com/noahgift/rust-multiplayer-roulette-game"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Client {
        #[clap(short, long, default_value = "Hello World")]
        message: String,
    },
    Server {},
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Client { message }) => {
            println!("Client connecting to server");
            rrgame::client(message).await?;
        }
        Some(Commands::Server {}) => {
            println!("Server listening");
            rrgame::server().await?;
        }
        None => {
            println!("Please specify a subcommand");
        }
    }
    Ok(())
}
