// calculate the area of shapes
enum Shape {
    Rectangle {width: u32, height: u32},
    Square(u32),
    Circle(f32),
}

impl Shape {
    fn area(&self) -> f32 {
        match *self {
            Shape::Rectangle {width, height} => (width * height) as f32,
            Shape::Square(ref s) => (s * s) as f32,
            Shape::Circle(ref r) => 3.14 * (r * r),
        }
    }
}


fn main() {
    
    // Shape Game

    let r = Shape::Rectangle{width: 10, height: 70};
    let s = Shape::Square(4);
    let c = Shape::Circle(8.5);

    println!("{}", r.area());
    println!("{}", s.area());
    println!("{}", c.area());
}

