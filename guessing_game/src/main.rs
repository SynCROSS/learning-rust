use rand::Rng;
use std::{cmp::Ordering, io::stdin};

fn main() {
    println!("Guessing Game -- Random Number");

    let start_number = 1;
    let end_number = 100;

    println!(
        "Selecting Number From {} To {} . . .",
        start_number, end_number
    );

    loop {
        let random_number = rand::thread_rng().gen_range(start_number, end_number);

        println!("Number Is Selected.");
        println!("Please Input Your Guess.");

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed To Read Line");

        // * Rust Allows To Shadow The Previous One Which Has Same Name.

        // * 'parse' Method Returns Enum Of 'Result' Type That Has Variants 'Ok' And 'Err',
        // * So 'match' Can Be Used Instead Of 'expect' Method.
        let guess = match guess.trim().parse::<u32>() {
            Ok(number) => number,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        match guess.cmp(&random_number) {
            Ordering::Greater => println!("It's Smaller Than Yours"),
            Ordering::Less => println!("It's Bigger Than Yours"),
            Ordering::Equal => {
                println!("It's Same Number, You Win");
                break;
            }
        }

        println!("Selected Number: {}", random_number);
        println!("Your Guess: {}", guess);
    }
}
