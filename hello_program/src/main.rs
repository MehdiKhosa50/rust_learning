use crate::function_module::function_module; // importing the function directly so we can call it without module prefix

// Importing our custom modules so we can use their functions
mod function_module;
mod string_module;
mod tuple_module;

fn main() {
    // Declare a 16-bit unsigned integer with a specific value
    let num: u16 = 65531;
    println!("The number is: {}", num);

    // Declare two numbers
    let num1: u8 = 254;
    let num2: u8 = 1;

    // Call our function that takes two numbers and returns their sum
    let sum = function_module(num1, num2);
    println!("The sum of {} and {} is: {}", num1, num2, sum);

    // Demonstrating Rust's integer min/max constants for different types
    println!("Max u8 value is: {}", u8::MAX);
    println!("Min u8 value is: {}", u8::MIN);

    println!("Max u16 value is: {}", u16::MAX);
    println!("Min u16 value is: {}", u16::MIN);

    println!("Max u32 value is: {}", u32::MAX);
    println!("Min u32 value is: {}", u32::MIN);

    println!("Max u64 value is: {}", u64::MAX);
    println!("Min u64 value is: {}", u64::MIN);

    println!("Hello, world!");

    // Call functions from custom modules
    string_module::print_hello();
    tuple_module::tuple_module();

    // Calling again (this one will print inside function_module itself)
    function_module::function_module(num1, num2);
}
