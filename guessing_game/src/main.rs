extern crate rand;

use std::env::args;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut lower: u32 = 1;
    let mut upper: u32 = 100;
    for arg in args() {
        let bounds: Vec<&str> = arg.split("-").collect();
        if bounds.len() == 2 {
            lower = match bounds[0].trim().parse() {
                Ok(num) => num,
                Err(_) => 1,
            };
            upper = match bounds[1].trim().parse() {
                Ok(num) => num,
                Err(_) => 100,
            };
            break;
        }
    }
    println!("Guess the number between {} and {}.", lower, upper);
    let secret_number = rand::thread_rng().gen_range(lower, upper+1);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
