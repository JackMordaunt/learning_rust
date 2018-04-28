use std::env;
use std::fs::File;
// prelude contains useful traits for doing io.
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let path = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", path);
    let mut f = File::open(path).expect("could not open file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("could not read the file");
    println!("With text:\n{}", contents);
}
