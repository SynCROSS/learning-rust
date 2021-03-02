fn main() {
    let v = vec![1, 2, 3];

    let v2 = v;

    println!("v[0] is: {}", v[0]); // ! Error, Because v's ownership moved to v2;
}
