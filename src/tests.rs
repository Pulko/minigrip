#[cfg(test)]
mod tests {
    use crate::{search, search_case_insensitive};

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
