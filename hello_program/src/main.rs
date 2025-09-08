// Importing our custom modules so we can use their functions
mod string_module;
mod tuple_module;

fn main() {
    // Declare a 16-bit unsigned integer with a specific value
    let num: u16 = 65531;
    println!("The number is: {}", num);

    // Demonstrating Rust's integer min/max constants for different types
    println!("Max u8 value is: {}", u8::MAX); // Maximum value for 8-bit unsigned integer
    println!("Min u8 value is: {}", u8::MIN); // Minimum value for 8-bit unsigned integer

    println!("Max u16 value is: {}", u16::MAX); // Maximum value for 16-bit unsigned integer
    println!("Min u16 value is: {}", u16::MIN); // Minimum value for 16-bit unsigned integer

    println!("Max u32 value is: {}", u32::MAX); // Maximum value for 32-bit unsigned integer
    println!("Min u32 value is: {}", u32::MIN); // Minimum value for 32-bit unsigned integer

    println!("Max u64 value is: {}", u64::MAX); // Maximum value for 64-bit unsigned integer
    println!("Min u64 value is: {}", u64::MIN); // Minimum value for 64-bit unsigned integer

    println!("Hello, world!"); // Just a greeting 🙂

    // Calling function from string_module (defined in string.rs)
    string_module::print_hello();

    // Calling function from tuple_module (defined in tuple.rs)
    tuple_module::tuple_module();
}
