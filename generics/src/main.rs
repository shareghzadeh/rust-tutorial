mod func;
mod find_largest;
// <T> stands for any type you want to use in "x"
#[derive(Debug)]
struct AllTypes<T> {
    x: T,
}

// you can use this impl for print the struct cleaner
// without using Debug

// impl<T> std::fmt::Display for AllTypes<T>
// where
//     T: std::fmt::Display,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.x)
//     }

// }



fn main() {
    // in genercs you can use all the types
    let s =  AllTypes { x: "Hello" };
    let s2 = AllTypes { x: String::from("Hello") };
    let i =  AllTypes { x: 2 };
    let f =  AllTypes { x: 3.5 };
    let c =  AllTypes { x: 'H' };


    println!("{:?}", s);
    println!("{:?}", s2);
    println!("{:?}", i);
    println!("{:?}", f);
    println!("{:?}", c);

    println!("\n");
    // func.rs file
    func::func();
    println!("\n");
    // find_largest.rs
    find_largest::larg();


}
