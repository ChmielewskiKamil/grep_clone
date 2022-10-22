use clap::Parser;

/// Search for a pattern in a file and display a line that contains it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    println!("Hello, world!");
}
