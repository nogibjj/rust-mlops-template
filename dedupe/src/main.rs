fn main() {
    if let Err(e) = dedupe::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
