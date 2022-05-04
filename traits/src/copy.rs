#[derive(Debug, Clone, Copy)]
struct A(i32);


pub fn copy() {
    let a = A(32);
    // with copy we dont need to add ".clone()"
    let c = a;

    println!("{:?}", a);


}