/*
command line tool that logs random fruits
*/
use clap::Parser;
extern crate log;

#[derive(Parser)]
#[command(name = "fruit-logger")]
#[command(author = "Noah Gift")]
#[command(version = "1.0")]
#[command(about = "logs random fruits", long_about = None)]
struct Cli {
    #[arg(long)]
    level: String,
}

fn main() {
    let args = Cli::parse();
    let level = args.level;
    let level = level.to_lowercase();
    match level.as_str() {
        "info" => {
            env_logger::builder()
                .filter_level(log::LevelFilter::Info)
                .init();
        }
        "trace" => {
            env_logger::builder()
                .filter_level(log::LevelFilter::Trace)
                .init();
        }
        "warn" => {
            env_logger::builder()
                .filter_level(log::LevelFilter::Warn)
                .init();
        }
        _ => {
            env_logger::builder()
                .filter_level(log::LevelFilter::Info)
                .init();
        }
    }
    clilog::random_fruit();
}
