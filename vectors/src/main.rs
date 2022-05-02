fn main() {
    // create vector
    let v1 = vec![1, 2, 3 ,4];

    // create vector
    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);

    println!("v1: {:?}\nv2: {:?}\n", v1, v2);
    // capaxity is how many memori this vector is using
    println!("v1 length: {}\nv1 capacity: {}", v1.len(), v1.capacity());
    println!("v2 last value: {:?}", v2.pop());



}
