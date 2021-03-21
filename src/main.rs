fn main() {
	// * But mutable references have one big restriction: 
	// * you can have only one mutable reference to 
	// * a particular piece of data in a particular scope.

	// * This restriction allows for mutation but in a very controlled fashion. 
	// * It’s something that a man stranger to Rust struggle with, 
	// * because most languages let you mutate whenever you’d like.

	// * The benefit of having this restriction is that Rust can prevent data races at compile time. 
	// * A data race is similar to a race condition and happens when these three behaviors occur:

	// * Two or more pointers access the same data at the same time.
	// * At least one of the pointers is being used to write to the data.
	// * There’s no mechanism being used to synchronize access to the data.
    
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
