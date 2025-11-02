use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,    
    pub ignore_case: bool,
}

const ARGS_COUNT: u8 = 3;

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() != 3 {
            return Err(format!("Incorrect number of arguments. Expected {ARGS_COUNT} but received {}", args.len()));
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        return Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: ignore_case,
        });
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut search: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            search.push(line);
        }
    }

    search
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut search = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            search.push(line);
        }
    }

    search
}