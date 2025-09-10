#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("Hello, world!");
    let p = Point { x: 10, y: 20 };
    println!("Point: {:?}", p);
    println!("Point x: {}", p.x);
    println!("Point y: {}", p.y);
}
