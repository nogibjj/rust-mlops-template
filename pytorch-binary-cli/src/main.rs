/*
A command-line tool that includes a PyTorch pretrained model and uses it to predict the class of an image.
*/
use clap::Parser;
use tch::vision::imagenet;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "Predicts the class of an image using a PyTorch pretrained model",
    after_help = "Example: "
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    Predict {
        #[clap(short, long)]
        image: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Predict { image }) => {
            let output = pytorch_binary_cli::predict_image(&image).unwrap();
            for (probability, class) in imagenet::top(&output, 5).iter() {
                println!("{:50} {:5.2}%", class, 100.0 * probability)
            }
        }
        None => println!("No command specified"),
    }
}
