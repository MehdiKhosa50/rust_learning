#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implement multiple methods for Rectangle
impl Rectangle {
    // Read-only method (borrow)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Borrow mutable to modify struct
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // Associated function (does not need instance)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("call read-only method -> Area: {}", rect1.area()); // call read-only method

    rect1.scale(2); // call mutable method
    println!("call mutable method -> Scaled Area: {}", rect1.area());

    let square = Rectangle::square(5); // call associated function
    println!("call associated function -> Square Area: {}", square.area());

    println!("{:?}", rect1); // ✅ Works because of Debug trait
}
