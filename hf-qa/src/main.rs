/*A command-line tool to do Q/A using Hugging Face models */
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Noah Gift")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Answer {
        #[clap(short, long)]
        question: String,
        #[clap(short, long)]
        context: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Answer { question, context }) => {
            let answer = hf_qa::answer_question(&question, &context);
            println!("Answer: {}", answer[0]);
        }
        None => println!("No command given"),
    }
}
