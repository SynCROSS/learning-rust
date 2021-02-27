use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        println!("-------------------------");
        println!("(1) Celsius to Fahrenheit");
        println!("(2) Fahrenheit to Celsius");
        println!("-------------------------");
        print!("Input: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Please Type a Number");

        let mut number = String::new();

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        if input == 1 {
            print!("Celsius: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut number)
                .expect("Please Type a Number");

            let celsius: f64 = number.trim().parse().unwrap();
            println!(
                "Celsius {} to Fahrenheit is {}",
                celsius,
                make_celsius_to_fahrenheit(celsius)
            );
        } else if input == 2 {
            print!("Fahrenheit: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut number)
                .expect("Please Type a Number");

            let fahrenheit: f64 = number.trim().parse().unwrap();
            println!(
                "Fahrenheit {} to Celsius is {}",
                fahrenheit,
                make_fahrenheit_to_celsius(fahrenheit)
            );
        } else {
            break;
        }
    }
}

fn make_celsius_to_fahrenheit(celsius: f64) -> f64 {
    let fahrenheit: f64 = celsius * 1.8 + 32.0;
    fahrenheit
}

fn make_fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    let celsius: f64 = (fahrenheit - 32.0) / 1.8;
    celsius
}
