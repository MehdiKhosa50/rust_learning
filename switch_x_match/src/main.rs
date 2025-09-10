fn main() {
    println!("Hello, world!");

    fn switch_x(x: i32) {
        match x {
            1 => println!("One"),
            2 => println!("Two"),
            3 | 4 | 5 => println!("Three, Four, or Five"),
            _ => println!("Something else"),
        }
    }
    switch_x(10);

    fn switch_y(y: char) {
        match y {
            'a'..='e' => println!("Between a and e"),
            'f'..='j' => println!("Between f and j"),
            _ => println!("Something else"),
        }
    }
    switch_y('a');

    let num: i32 = 4;

    fn is_even(num: i32) -> bool {
        if num % 2 == 0 {
            true
        } else {
            false
        }
    }

    println!("Is 4 even? {}", is_even(num));

    match is_even(num) {
        true => println!("It's even! from match statement"),
        false => println!("It's odd! from match statement"),
    }
}
