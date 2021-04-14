#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 10,
    };

    println!(
        "The area of the rectangle is {} x {} = {} square pixels.",
        rect1.width,
        rect1.height,
        rect1.area()
    );
}
