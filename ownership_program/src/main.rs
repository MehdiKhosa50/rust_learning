mod borrow;
fn main() {

    let my_string: String = get_string();
    println!("{}", my_string);

    let my_string2: String = String::from("Hello from main function!");
    println!("{}", my_string2);

    let my_string3: String = send_get_string(my_string2);
    println!("{}", my_string3);
    // println!("{}", my_string2); // this would cause a compile-time error because my_string2 is no longer valid


    // println!("Hello, world!");
    // let str = String::from("Hello from ownership program!"
    // );
    // println!("{}", str);
    // let str2 = str; // ownership moved here
    // println!("{}", str2);
    // println!("{}", str); // this would cause a compile-time error because str is no longer valid

    borrow::borrow_string();
}

fn get_string() -> String {
    let owned_string = String::from("Hello from get_string function!");
    return owned_string; // ownership moved to caller
}

fn send_get_string(received_string: String) -> String {
    return received_string;
}
