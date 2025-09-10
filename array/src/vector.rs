pub mod vector_module  {
    pub fn print_vector() {
        let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
        println!("Initial vector: {:?}", numbers);

        // Adding elements to the vector
        numbers.push(6);
        numbers.push(7);
        println!("Vector after pushes: {:?}", numbers);

        // Accessing elements
        if let Some(first) = numbers.get(0) {
            println!("First element: {}", first);
        }

        // Iterating over the vector
        for number in &numbers {
            println!("Number: {}", number);
        }

        // Removing the last element
        if let Some(last) = numbers.pop() {
            println!("Popped element: {}", last);
        }
        println!("Vector after pop: {:?}", numbers);
    }
}