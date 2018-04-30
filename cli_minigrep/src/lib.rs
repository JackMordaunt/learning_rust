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
    let lines = match cfg.case_sensitive {
        false => search_case_insensitive(&cfg.query, &contents),
        true => search(&cfg.query, &contents),
    };
    for line in lines {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn from_args(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("not enough arguments".to_string());
        }
        let mut case_sensitive = !env::var("MATCH_CASE").is_err();        
        let query = args[1].clone();
        let path = args[2].clone();
        let flags = args.iter().filter(|arg| arg.contains("-"));
        for f in flags {
            if f.contains("match") && f.contains("case") {
                case_sensitive = f.get_bool()?;
            }
        }
        Ok(Config{
            query,
            path,
            case_sensitive,
        })
    }
}

// Arg is an object that can extract a string or bool value from itself. 
trait Arg {
    type Error;
    fn get_string(&self) -> Result<String, Self::Error>;
    fn get_bool(&self) -> Result<bool, Self::Error>;
}

impl Arg for String {
    type Error = String;

    // --some-flag=hello -> Result<"hello">
    fn get_string(&self) -> Result<String, Self::Error> {
        let parts: Vec<&str> = self.split("=").collect();
        if parts.len() < 2 {
            return Ok("".to_string())
        }
        Ok(parts[1].to_string())
    }

    // --some-flag -> Result<true>
    // --some-flag=true -> Result<true>
    // --some-flag=false -> Result<false>
    // --some-flag=not-a-boolean -> Result<Self::Error>
    fn get_bool(&self) -> Result<bool, Self::Error> {
        match self.get_string() {
            Ok(s) => {
                match s.parse() {
                    Ok(b) => Ok(b),
                    Err(_) => Err(format!("argument {} should be 'true' or 'false'", self)),
                }
            },
            Err(_) => Ok(true),
        }
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

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut found = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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
        let args: Vec<String> = vec!["one".to_string(), "two".to_string()];
        if let Ok(_) =  Config::from_args(&args) {
            panic!("wanted error");
        }
    }

    #[test]
    fn config_enough_args() {
        let args: Vec<String> = vec!["exe".to_string(), "one".to_string(), "two".to_string()];
        if let Err(err) = Config::from_args(&args) {
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