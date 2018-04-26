// Bring io library into scope.
use std::io;
// You could bring in just the function like so:
// use std::io::stdin; 

fn main() {
    println!("Guess the number!");
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
    println!("You guess: {}", guess);
}

// [1] Associated functions are defined on a type rather than an instance of a 
//  type. I imagine it's like a static method on a java class.