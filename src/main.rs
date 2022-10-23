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

    let result = std::fs::read_to_string("test_file.txt");
    match result {
        Ok(content) => {
            println!("Content of the file: {}", content);
        }
        Err(error) => {
            println!("Can't open the file, here is the error: {}", error);
        }
    }
}
