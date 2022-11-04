use grep_clone::Config;
use std::{env, process};

fn main() {
    // argument handling //

    let args: Vec<String> = env::args().collect();

    // parse_config is only borrowing args now
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for: {}", config.search_query);
    println!("Path to search: {}", config.path_to_search);

    if let Err(e) = grep_clone::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
