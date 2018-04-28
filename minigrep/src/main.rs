extern crate minigrep;
use minigrep::Config;
use std::process;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args[1..]).unwrap_or_else(|err| {
        println!("Could not parse arguments: {}", err);
        process::exit(1);
    });
    if let Err(err) = minigrep::run(cfg) {
        println!("Application error: {}", err);
        process::exit(1);        
    }
}

