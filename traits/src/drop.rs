struct A {
    a: String,
}

impl Drop for A {
    fn drop(&mut self) {
        println!("dropped {}", self.a)
    }
}



pub fn drops() {
    let a = A{a: String::from("A")};
    {
        let b = A{a: String::from("B")};
        {
            let c = A{a: String::from("C")};
            drop(c);
            println!("leaving inner scope 2");
        }
        drop(b);
        println!("leaving inner scope 1");
    }
    drop(a);
    println!("program ending");

}