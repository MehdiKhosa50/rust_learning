fn main() {
    println!("Hello, world!");

    struct Circle {
        radius: f64,
    }

    trait Shape {
        fn area(&self) -> f64;
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    let c = Circle { radius: 5.0 };
    println!("Circle area: {:.2}", c.area());
}
