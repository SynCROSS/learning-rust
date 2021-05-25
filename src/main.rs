fn main() {
    let f = std::fs::File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    println!("{:?}", f);
}
