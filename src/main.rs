// * If we have a string slice, we can pass that directly.
// * If we have a String, we can pass a slice of the entire String.
// * Defining a function to take a string slice instead of a reference
// * to a String makes our API more general and useful without losing any functionality

fn main() {
    let my_string = String::from("hello world");

    // * first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    println!("{}", word);

    let my_string_literal = "hello world";
    // * first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);

    // * Because string literals *are* string slices already,
    // * this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
