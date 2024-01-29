mod config;
mod tests;

use std::error::Error;
use std::fs;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let config::Config {
        query,
        file_path,
        ignore_case,
    } = config;

    let contents = fs::read_to_string(&file_path)?;

    let search_function: for<'a> fn(&'a str, &'a str) -> Vec<&'a str> = {
        if ignore_case {
            search
        } else {
            search_case_insensitive
        }
    };

    for line in search_function(&query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn build(args: impl Iterator<Item = String>) -> Result<config::Config, &'static str> {
    config::Config::build(args)
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect::<Vec<&str>>()
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect::<Vec<&str>>()
}
