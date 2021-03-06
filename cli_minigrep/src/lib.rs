pub mod arg;
pub mod config;

use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
pub use config::Config;

// Box is a trait object; it allows us to return an object that automatically 
// satisfies the Error trait without use needing to be concrete about it.
// This gives us flexibility to return error values that may be of different
// types in different error cases.
pub fn run(cfg: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(cfg.path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let lines = match cfg.case_sensitive {
        false => search_case_insensitive(&cfg.query, &contents),
        true => search(&cfg.query, &contents),
    };
    for line in lines {
        println!("{}", line);
    }
    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_not_enough_args() {
        let args: Vec<String> = vec!["one".to_string(), "two".to_string()];
        if let Ok(_) =  Config::from_args(args) {
            panic!("wanted error");
        }
    }

    #[test]
    fn config_enough_args() {
        let args: Vec<String> = vec!["exe".to_string(), "one".to_string(), "two".to_string()];
        if let Err(err) = Config::from_args(args) {
            panic!(format!("unexpected error: {}", err));
        }
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}