use std::process;
use std::env;
use std::fs::File;
use std::error::Error;
// prelude contains useful traits for doing io.
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args[1..]).unwrap_or_else(|err| {
        println!("Could not parse arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", cfg.query);
    println!("In file {}", cfg.path);
    if let Err(err) = run(cfg) {
        println!("Application error: {}", err);
        process::exit(1);        
    }
}

// Box is a trait object; it allows us to return an object that automatically 
// satisfies the Error trait without use needing to be concrete about it.
// This gives us flexibility to return error values that may be of different
// types in different error cases.
fn run(cfg: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(cfg.path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("With text:\n{}", contents);
    Ok(())
}

struct Config {
    query: String,
    path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let query = args[0].clone();
        let path = args[1].clone();
        Ok(Config{ query, path })
    }
}