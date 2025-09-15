// Implement the macro to show how they works behind the scenes

macro_rules! my_vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    println!("Hello, world!");

    my_vec_macro();
}

fn my_vec_macro() {
    let v = my_vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
}

// The above macro `my_vec!` works similarly to the built-in `vec!` macro in Rust.
// It takes a variable number of expressions and creates a vector containing those expressions.
// For example, `my_vec![1, 2, 3]` will expand to code that creates a vector with the elements 1, 2, and 3.
// You can test it by running the `my_vec_macro` function in the `main` function.
// Note: Macros in Rust are expanded at compile time, so they can be a bit tricky to debug.
// You can use `cargo expand` to see the expanded code of your macros.
// To use `cargo expand`, you need to add it as a cargo subcommand. You can do this by running:
// cargo install cargo-expand
// Then, you can run `cargo expand` in your project directory to see the expanded code.
// For more information on macros, you can refer to the official Rust documentation:
// https://doc.rust-lang.org/book/ch19-06-macros.html
// https://doc.rust-lang.org/reference/macros-by-example.html
// https://crates.io/crates/cargo-expand
