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
}
