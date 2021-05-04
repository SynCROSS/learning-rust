fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = std::collections::HashMap::new();
    map.insert(field_name, field_value);
    // * field_name and field_value are invalid at this point, try using them and
    // * see what compiler error you get!

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}
