// using trait in function

// create our trait
trait MyTrait {
    fn sum(&self) -> u32;
}

// create our struct
struct MyStruct {
    size: u32,
}

// implementing MyTrait for MyStruct
impl MyTrait for MyStruct {
    fn sum(&self) -> u32 {
        self.size
    }
}

// using our trait in function as parameter
// and then using "sum()" method
fn print_sum(a: &impl MyTrait) {
    println!("{}", a.sum());
}



pub fn func() {
    let my_struct = MyStruct{ size: 10 };

    print_sum(&my_struct);
}