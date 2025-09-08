pub fn print_hello() {
    // string literal (&'static str)
    let hello = "Hello, world! from string_module to test string literal";

    // String object (heap allocated, growable, owned)
    let hello2 = String::from("Hello, world! from string_module to test String object");

    // converting a &str → String (heap owned copy)
    let hello3 = hello.to_string();

    // borrowing a String as a &str (view into the String)
    let hello4: &str = &hello2;

    println!("{}", hello4); // prints borrowed &str view of hello2
    println!("{}", hello); // prints string literal
    println!("{}", hello2); // prints String
    println!("converted hello to string {}", hello3);
}
