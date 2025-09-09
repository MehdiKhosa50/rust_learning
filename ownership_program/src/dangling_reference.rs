pub fn dangling_reference() {
    let r;
    {
        let x = 42;
        r = &x; // dangling reference

    }
    // println!("{}", r); // this would cause a compile-time error
}
