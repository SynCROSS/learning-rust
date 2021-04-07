fn main() {
    struct Person {
        name: String,
        age: i8,
    }

    let person1 = Person {
        name: String::from("SynCROSS"),
        age: 18,
    };

    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age)
}
