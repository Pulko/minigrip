use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough parameters to perform a search");
        }

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: args.contains(&String::from("--ignore-case")),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let Config {
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn empty_case() {
        let query = "SOMETHING";
        let contents = "";

        assert_eq!(Vec::<&str>::new(), search(query, contents));
        assert_eq!(Vec::<&str>::new(), search_case_insensitive(query, contents));
    }
}
