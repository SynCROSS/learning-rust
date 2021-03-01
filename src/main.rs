use std::io::{self, Write};

fn main() {
    print!("Length: ");
    io::stdout().flush().unwrap();
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Error has Occurred.");
    let length = match input2.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Error: {}", e);
            0
        }
    };

    fibonacci(length);
}

fn fibonacci(num: i64) {
    let mut pre_tmp = 1;
    let mut tmp = 1;
    let mut i = 0;

    loop {
        i = tmp + pre_tmp;
        println!("{} {} {}", pre_tmp, tmp, i);
        pre_tmp = tmp;
        tmp = i;
        if i > num {
            break;
        };
    }
}
