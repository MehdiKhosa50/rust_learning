fn main() {
    println!("Hello, world!");

    iterator();

}


fn iterator () {
    let numbers = vec![1, 2, 3, 4, 5];
    // let mut iter  = numbers.iter();  // this will create an iterator that borrows numbers
    let mut iter  = numbers.into_iter(); // this will create an iterator that takes ownership of numbers

    while let Some(num) = iter.next() {
        println!("{}", num);
    }
    println!("Iterator has been fully consumed.");
    println!("{:?}", iter.next());
    // println!("{:?}", numbers); // This will cause a compile-time error


}