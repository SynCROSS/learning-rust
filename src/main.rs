// #![allow(unused)]
fn main() {
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("Red: {}, Green: {}, Blue: {}", &black.0, &black.1, &black.2)
}
