// Bring in rand crate.
extern crate rand;

// Bring io library into scope.
use std::io;
use std::cmp::Ordering;
// You could bring in just the function like so:
// use std::io::stdin; 
use rand::Rng;

fn main() {
    println!("Guess the number!");
    // ::thread_rng uses a thread local seed.
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Please input your guess.");
    // Create a place to store the user input.
    // String is a growable UTF-8 encoded bit of text.
    // ::new is an associated function [1].
    let mut guess = String::new();
    // We pass in a mutable reference to the mutable String object.
    // read_line accepts a reference because it intends to write the line into
    // provided String, if we passed by value the function would receive a copy
    // of String which means we would not see the data being written to it. 
    // Since references are immutable by default we need tell Rust that it can
    // be mutated.  
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    // String.parse() parses a string into a number. The number is determined by
    // the type we are assigning to, u32 in this case. 
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

// [1] Associated functions are defined on a type rather than an instance of a 
//  type. I imagine it's like a static method on a java class.