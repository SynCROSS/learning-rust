fn main() {
    let f = std::fs::File::open("hello.txt").expect("Failed to open hello.txt");
    
    println!("{:?}", f);
}
