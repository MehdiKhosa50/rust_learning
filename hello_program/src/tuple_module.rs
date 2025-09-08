pub fn tuple_module() {
    // A tuple is a fixed-size collection of values of potentially different types.
    let my_tuple: (i32, f64, char) = (42, 3.14, 'a'); // tuple with int, float, char

    // Tuple containing a String and an unsigned 8-bit integer
    let tuple: (String, u8) = (String::from("Hello, Tuple!"), 255);
    println!("Tuple String: {:?}", tuple); // {:?} for debug printing of tuple

    // Tuple with unsigned int, string slice (&str), and a boolean
    let your_tuple: (u32, &str, bool) = (7, "hello", true);
    println!("Your tuple: {:?}", your_tuple);

    // Tuple with mixed data types (int, str, float, char, bool)
    let mixed_tuple = (1, "two", 3.0, '4', false);
    println!("Mixed tuple: {:?}", mixed_tuple);

    // Destructuring the tuple into individual variables
    let (a, b, c, d, e) = mixed_tuple;
    println!(
        "Destructured mixed tuple: a = {}, b = {}, c = {}, d = {}, e = {}",
        a, b, c, d, e
    );

    // Accessing tuple elements by index (0-based)
    let first_element = my_tuple.0; // 42
    let second_element = my_tuple.1; // 3.14
    let third_element = my_tuple.2; // 'a'

    println!("First element: {}", first_element);
    println!("Second element: {}", second_element);
    println!("Third element: {}", third_element);

    // Destructuring a tuple directly into variables
    let (x, y, z) = my_tuple;
    println!("Destructured: x = {}, y = {}, z = {}", x, y, z);
}
