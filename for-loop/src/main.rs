fn main() {
    let list: Vec<u32> = vec![10, 20, 30 ,40, 50];
    // "i" interate trough the list and get the
    // elements of list one by on
    for i in list {
        println!("{}", i);
    }

    
    let sr: &str = "String";
    // if you want to get the character
    // of a string you have to ues
    // "chars()" method
    for j in sr.chars() {
        println!("{}", j);
    }


    // print the range of 1 to 100
    // we put "1..101" cus "1..101-1"
    // we call this exclusive range
    for i in 1..11 {
        println!("{}", i);
    }
    // this is inclusive
    // "..=" includes the 11
    for i in 1..=11 {
        println!("{}", i);
    }
}
