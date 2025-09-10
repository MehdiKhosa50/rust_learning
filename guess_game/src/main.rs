use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::io; // <-- For unique guesses

fn main() {
    println!("Welcome to the Guessing Game!");

    let secret_number = rand::rng().random_range(1..=100);

    if cfg!(debug_assertions) {
        println!("(Debug mode: The secret number is {secret_number})");
    }

    // Track all guesses and attempts
    let mut attempts = 0;
    let mut guesses = HashSet::new();

    loop {
        if guesses.len() == 100 {
            println!("All 100 numbers have been guessed. Game over!");
            println!("The secret number was: {secret_number}");
            break;
        }

        println!("Please input your guess (1-100), or type 'exit' to quit:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess = guess.trim();

        if guess.eq_ignore_ascii_case("exit") || guess.eq_ignore_ascii_case("quit") {
            println!("Exiting the game. Goodbye!");
            break;
        }

        // Parse the guess into a number
        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // ✅ Check range
        if guess < 1 || guess > 100 {
            println!("Your guess must be between 1 and 100!");
            continue;
        }

        // ✅ Check uniqueness
        if !guesses.insert(guess) {
            println!("You already guessed {guess}! Try something new.");
            continue;
        }

        // ✅ Increment attempts
        attempts += 1;
        println!("Attempt #{attempts}");
        println!("You guessed: {guess}");
        println!("Unique guesses so far: {:?}", guesses);
        println!("Guesses remaining: {}", 100 - guesses.len());
        println!("-----------------------------------");

        // ✅ Compare with secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("🎉 You win! It took you {attempts} attempts.");
                break;
            }
        }
    }
}
