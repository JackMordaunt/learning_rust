use std::fs::File;
use std::error::Error;
// prelude contains useful traits for doing io.
use std::io::prelude::*;
use std::env;

// Box is a trait object; it allows us to return an object that automatically 
// satisfies the Error trait without use needing to be concrete about it.
// This gives us flexibility to return error values that may be of different
// types in different error cases.
pub fn run(cfg: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(cfg.path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    for line in search(&cfg.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub path: String,
}

impl Config {
    pub fn from_args(args: env::Args) -> Result<Config, String> {
        let mut args: Vec<String> = args.collect();
        args.remove(0);
        if args.len() < 2 {
            return Err("not enough arguments".to_string());
        }
        let query = args.remove(0);
        let path = args.remove(0);
        Ok(Config{ query, path })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut found = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            found.push(line);
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_not_enough_args() {
        let args: Vec<String> = vec!["one".to_string()];
        if let Ok(_) =  Config::new(&args) {
            panic!("wanted error");
        }
    }

    #[test]
    fn config_enough_args() {
        let args: Vec<String> = vec!["one".to_string(), "two".to_string()];
        if let Err(err) = Config::new(&args) {
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
}