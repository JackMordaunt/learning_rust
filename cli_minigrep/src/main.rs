extern crate minigrep;
use std::process;
use std::env;


fn main() {
    let cfg = minigrep::Config::from_args(env::args()).unwrap_or_else(|err| {
        println!("Could not parse arguments: {}", err);
        process::exit(1);
    });
    if let Err(err) = minigrep::run(cfg) {
        println!("Application error: {}", err);
        process::exit(1);        
    }
}