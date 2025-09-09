fn main() {
    println!("Hello, world!");
    let str = String::from("Hello from ownership program!"
    );
    println!("{}", str);
    let str2 = str; // ownership moved here
    println!("{}", str2);
    // println!("{}", str); // this would cause a compile-time error because str is no longer valid
}
