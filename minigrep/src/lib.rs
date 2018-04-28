use std::fs::File;
use std::error::Error;
// prelude contains useful traits for doing io.
use std::io::prelude::*;

// Box is a trait object; it allows us to return an object that automatically 
// satisfies the Error trait without use needing to be concrete about it.
// This gives us flexibility to return error values that may be of different
// types in different error cases.
pub fn run(cfg: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(cfg.path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("With text:\n{}", contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let query = args[0].clone();
        let path = args[1].clone();
        Ok(Config{ query, path })
    }
}