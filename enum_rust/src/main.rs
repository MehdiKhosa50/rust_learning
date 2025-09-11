mod simple_enum;
mod complex_enum;
mod optional_enum;
mod result_enum;
fn main() {
    println!("Hello, world!");

    simple_enum::run();
    complex_enum::run();
    optional_enum::run();
    result_enum::run();
}
