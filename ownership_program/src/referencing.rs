pub fn referencing() {
    let mut x = 5;
    println!("x = {}", x);

    // ✅ Multiple immutable borrows are fine
    let r1 = &x;
    let r2 = &x;
    println!("Immutable borrows: {}, {}", r1, r2);

    // ✅ NLL: After r1 and r2 are used above, they're no longer needed,
    // so we can safely borrow mutably now.
    let r3 = &mut x;
    *r3 += 1;
    println!("Mutable borrow after immutable borrows: {}", r3);

    // ❌ Conflict example: uncommenting this will fail
    // println!("Still using r1: {}", r1); 
    // If you uncomment above, Rust will complain:
    // "cannot borrow `x` as mutable because it is also borrowed as immutable"

    println!("Final x = {}", x);
}