use std::path::{PathBuf, Path};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The file to parse.
    filename: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    println!("Got filename: {}", cli.filename.display());
}
