use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("Pattern: {:?}, Path: {:?}", args.pattern, args.path);
}