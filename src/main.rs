fn main() {
	let mut s = String::from("hello");

	println!("{}", change(&mut s));
}

// * Mutable References
fn change(some_string: &mut String) -> &mut String {
	some_string.push_str(", world");

	some_string
}
