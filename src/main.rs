extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 100 + 1);
    println!("The Secret Number is {}", secret_number);
    loop {
        let mut number = String::new();
        println!("Guess A Number:");

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to Read Line");

        // * Rust allows us to shadow the previous value with a new one
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        println!("You Guessed : {}", number);

        match number.cmp(&secret_number) {
            Ordering::Less => println!("It's Small."),
            Ordering::Equal => {
                println!("Great, You're a Winner.");
                break;
            }
            Ordering::Greater => println!("It's Big."),
        }
    }
}
