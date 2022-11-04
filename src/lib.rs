use std::{error::Error, fs};
pub struct Config {
    pub search_query: String,
    pub path_to_search: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // text file handling //
    let contents = fs::read_to_string(config.path_to_search)?;

    println!("Found following content: \n{contents}");
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
