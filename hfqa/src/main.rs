//A command-line tool to do Question and Answer with Hugging Face
use clap::Parser;
extern crate anyhow;
use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};

#[derive(Parser)]
#[clap(version = "1.0", author = "Noah Gift", about = "Question and Answer with Hugging Face")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    Question {
        #[clap(short, long)]
        qname: String,
        context: String,
    },
}

fn logic(qname: String, context: String) -> anyhow::Result<()> {
    let qa_model = QuestionAnsweringModel::new(Default::default())?;
                                                        
let question = qname;
let context = context;
let answers = qa_model.predict(&[QaInput { question, context }], 1, 32);
for answer in answers {
    println!("Answer: {:?}", answer);
}
Ok(())
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Question { qname, context }) => {
            logic(qname, context).unwrap();
        }
        None => println!("No subcommand was used"),
    }
}