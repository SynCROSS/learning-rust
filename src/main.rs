extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 100 + 1);
    println!("The Secret Number is {}", secret_number);

    let mut number = String::new();
    println!("Guess A Number:");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to Read Line");

    // * Rust allows us to shadow the previous value with a new one
    let number: u32 = number.trim().parse().expect("Please Type a Number!");
    println!("You Guessed : {}", number);

    match number.cmp(&secret_number) {
        Ordering::Less => println!("It's Small."),
        Ordering::Equal => println!("Great, You're a Winner."),
        Ordering::Greater => println!("It's Big."),
    }
}
