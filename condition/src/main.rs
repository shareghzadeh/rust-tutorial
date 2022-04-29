fn main() {
    // simple condition

    let number = 4;

    if number % 4 == 0 {
        println!("{} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("{} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("{} is divisible by 2", number);
    } else {
        println!("{} is not divisible by 4, 3, 2", number);
    }


    // let condition

    let b = 2;

    // both return types in if and else has to be same type
    let con = if b == 2 {
        "its is 2"  // string
    } else {
        "its not 2" // string
    };

    println!("b: {}", con);
}
