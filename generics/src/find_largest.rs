// we use generics in function cus the perameter could be any type
// if you dont want to use genrics in this function you have to write two function
// one for "i32" datatype and one for "char"
// but with generics we only write one function
// soooo use generics :)


fn largest_value<T: std::cmp::PartialOrd + Copy> (list: &[T]) -> T {
    // you can use:
    // where T: std::cmp::PartialOrd + Copy
    // instead of <T: std::cmp::PartialOrd + Copy> 
    
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}



pub fn larg() {
    let number_list = vec![23,12,56,17,50,60,70];
    let char_list = vec!['a', 'b', 'd', 'n', 'q'];

    let res_number = largest_value(&number_list);
    let res_char = largest_value(&char_list);

    println!("largest number is: {}", res_number);
    println!("largest char is: {}", res_char);


    

}