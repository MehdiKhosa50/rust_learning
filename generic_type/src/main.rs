// use std::fmt::Display;
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = true;
    let z = "Hello, Rust!".to_string();
    let a = vec![1, 2, 3, 4, 5];
    let b = (10, "Tuple", 3.14);

    print_type_of(&x); // i32
    print_type_of(&y); // bool
    print_type_of(&z); // alloc::string::String
    print_type_of(&a); // alloc::vec::Vec<i32>
    print_type_of(&b); // (i32, &str, f64)
}
