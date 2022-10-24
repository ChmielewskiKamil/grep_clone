use clap::Parser;

/// Search for a pattern in a file and display a line that contains it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// A path to a file in which to search
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "test_file.txt";
    let content = std::fs::read_to_string(path)?;

    println!("Content of the file: {}", content);

    Ok(())
}
