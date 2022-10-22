use clap::Parser;

/// Search for a pattern in a file and display a line that contains it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// A path to a file in which to search
    path: std::path::PathBuf,
}

fn main() {
    // parse method is meant to be used in the main function
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("Could not read the file :(");
}
