use std::fs::File;
use std::io;
use std::io::Write;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    println!("Please enter some input: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("You entered: {}", input.trim());
    let path = "output.txt";
    let mut file = File::create(path).expect("Could not create file");
    file.write_all(input.as_bytes()).expect("Could not write to file");
    println!("Input written to {}", path);
}
