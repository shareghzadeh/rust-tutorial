struct Fib {
    a: u32,
    b: u32,
}

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let n = self.a + self.b;
        self.a = self.b;
        self.b = n;

        Some(self.a)

    }
}


fn fib() -> Fib {
    Fib{
        a: 1,
        b: 1,
    }
}

pub fn fibo() {

    // takes the 10 first fibo
    for i in fib().take(10) {
        println!("{}", i)
    }

    println!("\n");

    // skip the 14 first fibo and take 10 after that
    for j in fib().skip(14).take(10) {
        println!("{}", j)
    }

    // useing "next()" method
    let mut f = fib();
    println!("{:?}", f.next());
    println!("{:?}", f.next());
    println!("{:?}", f.next());
    println!("{:?}", f.next());
    println!("{:?}", f.next());
    println!("{:?}", f.next());
}