// Option is a basic enum for error handeling 

fn division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let res = division(5.4, 6.7);
    match res {
        // "{:.3}" decrese decimal numbers to 3
        Some(x) => println!("{:.3}", x),
        None    => println!("can't divide by 0"),
    }

}
