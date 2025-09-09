pub fn write_borrow() {
    let mut my_string: String = String::from("Hello");
    println!("Before mutation: {}", my_string);

    // Mutable borrow of my_string
    mutate_string(&mut my_string);

    println!("After mutation: {}", my_string);
}

fn mutate_string(s: &mut String) {
    s.push_str(", world!"); // Mutate the string by appending text
}
