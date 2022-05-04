mod clone;
mod copy;
mod drop;
mod fibo;

trait Shape {
    fn area(&self) -> u32;
}

struct Rectangle {
    x: u32,
    y: u32,
}

struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.x * self.y
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        (self.radius * self.radius * 3.14) as u32
    }
}


fn main() {
    let c = Circle {
        radius: 12.2,
    };
    let r = Rectangle {
        x: 30,
        y: 20,
    };
    println!("{} {}", c.area(), r.area());

    // clone.rs file
    clone::clone();
    println!("\n");
    // copy.rs file
    copy::copy();
    println!("\n");
    // drop.rs file
    drop::drops();
    println!("\n");
    fibo::fibo();
    println!("\n");


}
