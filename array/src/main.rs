fn main() {
    println!("Hello, world!");
    let mut array = [1, 2, 3, 4, 5];
    println!("Original array: {:?}", array);

    // Immutable borrow of the array
    let first_element = &array[0];
    println!("First element (immutable borrow): {}", first_element);

    // Mutable borrow of the array
    let array_mut = &mut array;
    array_mut[1] = 20;
    println!("Array after mutable borrow: {:?}", array_mut);
    array_mut.sort();

    println!("Array after mutable borrow and sort: {:?}", array_mut);
    // array_mut.push_str("6"); // This line will cause a compile-time error because arrays have fixed size
}
