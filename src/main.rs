fn main() {
    // * Except For the Rust Primitive Types,
    // * The Rest(Vec, String, etc) Have NO Copy Trait.
    // * Copy Trait is Copyable Trait.

    let v = vec![1, 2, 3];

    let v2 = &v; // * However, if it was assigned an reference, there is no error.

    println!("v[0] is: {}", v[0]); // * v[0] is: 1
}
