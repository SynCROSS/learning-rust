fn print_string(string: &str) {
    for c in string.chars() {
        print!("{}", c);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }
    println!();
}

fn main() {
    let hellos = [
        "السلام عليكم",
        "Dobrý den",
        "Hello",
        "שָׁלוֹם",
        "नमस्ते",
        "こんにちは",
        "안녕하세요",
        "你好",
        "Olá",
        "Здравствуйте",
        "Hola",
    ];

    for hello in hellos.iter() {
        print_string(hello);
    }
}
