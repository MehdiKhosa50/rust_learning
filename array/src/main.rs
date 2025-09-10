use crate::vector::vector_module; // importing the function directly so we can call it without module prefix

mod vector;
mod array_reference;

fn main() {
    println!("Hello, world!");
    let mut array = [1, 2, 3, 4, 5];
    println!("Original array: {:?}", array);

    // Immutable borrow of the array
    let first_element = &array[0];
    println!("First element (immutable borrow): {}", first_element);
    println!("Array after immutable borrow: {:?}", array);

    // Mutable borrow of the array
    let array_mut = &mut array;
    array_mut[1] = 20;
    println!("Array after mutable borrow: {:?}", array_mut);
    array_mut.sort();

    println!("Array after mutable borrow and sort: {:?}", array_mut);
    // let second_element = &array[1]; // Immutable borrow after mutation
    // println!("Second element (after mutation): {}", second_element); // this will cause a compile-time error if uncommented
    println!("Final array: {:?}", array_mut);
    // array_mut.push_str("6"); // This line will cause a compile-time error because arrays have fixed size

    // vector::vector_module::print_vector();
    vector_module::print_vector();
    array_reference::array_reference();

}
