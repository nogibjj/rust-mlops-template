/*Command-line interface for ONNX */
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "Command-line interface for ONNX"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    Infer {},
}

//invoke lib.rs using onnx_demo namespace
//use onnx_demo::run;
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Infer {}) => {
            let result = onnx_demo::run();
            //format result using non-standarf formatting
            println!("{:#?}", result);
        }
        None => println!("No subcommand was used"),
    }
}
