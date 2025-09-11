enum Shape {
    Circle(f64), // radius
    Rectangle(f64, f64), // width, height
    Triangle(f64, f64, f64), // side1, side2, side3
}

enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Self::Quit => println!("Quit message received."),
            Self::Move { x, y } => println!("Move to coordinates: ({x}, {y})"),
            Self::Write(text) => println!("Write message: {text}"),
            Self::ChangeColor(r, g, b) => println!("Change color to RGB({r}, {g}, {b})"),
        }
    }
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Self::Circle(radius) => std::f64::consts::PI * radius.powi(2),
            Self::Rectangle(width, height) => width * height,
            Self::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

pub fn run() {
    eprintln!("Running complex_enum module...");

    let shapes = [Shape::Circle(5.0), Shape::Rectangle(4.0, 6.0), Shape::Triangle(3.0, 4.0, 5.0)];

    for shape in &shapes {
        println!("Area: {:.2}", shape.area());
    }

    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write("Hello, Rust!".into()),
        Message::ChangeColor(255, 0, 0),
    ];

    for msg in &messages {
        msg.call();
    }
}
