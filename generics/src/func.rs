use std::fmt;



fn gen_func<T: fmt::Debug> (x: T) {
    println!("{:?}", x)
}


pub fn func() {
    gen_func(2);
    gen_func("Hi");
    gen_func('C');
    gen_func(String::from("Hello"));
}