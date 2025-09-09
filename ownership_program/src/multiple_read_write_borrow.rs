pub fn multiple_read_write_borrow() {
    let mut s = String::from("hello");
    let r1 = &s; // immutable borrow
    let r2 = &s; // another immutable borrow
    println!("{}, {}", r1, r2);
    let r3 = &mut s; // mutable borrow
    r3.push_str(", world");
    println!("{}", r3);
    println!("{}", s);
    // println!("{}", r1); // this would cause a compile-time error because r1 is still in use
    println!("multiple_read_write_borrow function executed");
}