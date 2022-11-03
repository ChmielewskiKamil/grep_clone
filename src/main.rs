use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // args[1] is the name of the program, we don't need that
    let search_query = &args[1];
    let path_to_search = &args[2];

    println!("Searching for: {}", search_query);
    println!("Path to search: {}", path_to_search)
}
