use std::fmt::{ Display, Formatter, Result };

// we add this for print our object without any method or function
#[derive(Debug)]


struct Object {
    height: u32,
    width: u32,
}




// this is impl for Methods
impl Object {
    // this method will multiply height and the width 
    fn area(&self) -> u32{
        &self.height * &self.width
    }

    // this method will get the height in object
    fn get_hight(&self) -> &u32{
        &self.height
    }

    // this method will get the width in object
    fn get_width(&self) -> &u32{
        &self.width
    }


    // this method will show the the height, width and the area of them
    fn show(&self) {
        println!("{}x{} = {}", &self.height, &self.width, &self.area());
    }
}


// this is impl for Related Functions
impl Object {
    fn new(height: u32, width: u32) -> Object {
        Object {
            height,
            width,
        }
    }
}

// we impl this to we can don't use {:?}
impl Display for Object {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", &self.height, &self.width)

    }
}


fn main() {
    // Create object without new() function
    let o = Object {
        height: 20,
        width: 10,
    };
    // print the area of "o" object with area method
    println!("{}x{} = {}", o.width, o.height, o.area());

    // Create object with new() function
    let obj = Object::new(32, 100);

    // Get the height of "obj" object
    println!("{}", obj.get_hight());

    // Get the width of "obj" object
    println!("{}", obj.get_width());

    // using show method
    o.show();
    // print the object without any method or function
    println!("{:?}", o);
    // Print the object without debug mode like {:?}
    println!("{}", o);
}