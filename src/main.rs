use std::{env, fs};

fn main() {
    // argument handling //

    let args: Vec<String> = env::args().collect();

    let (search_query, path_to_search) = parse_config(&args);

    println!("Searching for: {}", search_query);
    println!("Path to search: {}", path_to_search);

    // text file handling //

    let contents = fs::read_to_string(path_to_search).expect("Expected to read the file");

    println!("Found following content: \n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let search_query = &args[1];
    let path_to_search = &args[2];

    (search_query, path_to_search)
}
