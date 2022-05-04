#[derive(Debug, Clone)]
struct A(i32);

pub fn clone() {
    let a = A(20);
    // if we dont use clone() we cant print variable "a"
    // cus it moves to variable "c"
    // but with clone() we just clone the value
    let c = a.clone();

    println!("{:?}", a);



    

}