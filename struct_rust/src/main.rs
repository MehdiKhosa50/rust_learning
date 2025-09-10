mod rectangle;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, world!");
    let p = Point { x: 10, y: 20 };
    println!("Point: {:?}", p);
    println!("Point x: {}", p.x);
    println!("Point y: {}", p.y);

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
    println!("Area: {}", area(10, 300));

    fn area2(rect: &Rectangle) -> u32 {
        rect.width * rect.height
    }
    let rect = Rectangle {
        width: 10,
        height: 200,
    };
    println!("Rectangle area: {}", area2(&rect));

    println!("Area: {}", rectangle::rectangle_area(30, 50));
    let rect1 = Rectangle {
        width: 10,
        height: 200,
    };
    println!("Rectangle area: {}", rectangle::rectangle_area(rect1.width, rect1.height));
}
