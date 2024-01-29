pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let ignore_case = match args.next() {
            Some(arg) => arg == String::from("--ignore-case"),
            None => false,
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
