use std::{env, fs, process};

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

    // text file handling //

    let contents = fs::read_to_string(config.path_to_search).expect("Expected to read the file");

    println!("Found following content: \n{contents}");
}

struct Config {
    search_query: String,
    path_to_search: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let search_query = args[1].clone();
        let path_to_search = args[2].clone();

        Ok(Config {
            search_query,
            path_to_search,
        })
    }
}
