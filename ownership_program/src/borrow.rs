pub fn borrow_string() {
    let my_string: String = String::from("Hello from borrow_string function!");
    println!("{}", my_string);

    // let length = calculate_length(my_string); // ownership moved to calculate_length

    let length: usize = calculate_length(&my_string); // borrow my_string, ownership not moved

    // copy the length value (usize is a Copy type) very expensive to copy the whole String

    println!("The length of '{}' is {}.", my_string, length);
}

fn calculate_length(len: &String) -> usize {
    let length: usize = len.len();
    return length;
}
