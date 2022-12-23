//Searches a path for duplicate files
use clap::Parser;

#[derive(Parser)]
#[command(name = "deduper")]
#[command(author = "Noah Gift")]
#[command(version = "1.0")]
#[command(about = "Finds duplicate files on the filesystem", long_about = None)]
struct Cli {
    #[arg(long)]
    path: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Searching path: {:?}", cli.path);
    let files = dedupe::walk(&cli.path).unwrap();
    println!("Found {} files", files.len());
    //Now find duplicates
    let checksums = dedupe::checksum(files).unwrap();
    let duplicates = dedupe::find_duplicates(checksums);
    println!("Found {} duplicates", duplicates.len());
    for duplicate in duplicates {
        println!("Duplicate files: {:?}", duplicate);
    }
}
