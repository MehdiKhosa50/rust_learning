pub fn run() {
    println!("Running result_enum module...");

    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err("Cannot divide by zero".into())
        } else {
            Ok(numerator / denominator)
        }
    }

    let results = [divide(10.0, 2.0), divide(5.0, 0.0), divide(9.0, 3.0)];

    for result in &results {
        match result {
            Ok(value) => eprintln!("Result: {}", value),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
