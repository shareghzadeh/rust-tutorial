use std::fs::File;

fn main() {
    let mut f = File::create("/home/hamid/Programming/Rust/Tutorial/result/test.txt");
    let mut f = File::open("/home/hamid/Programming/Rust/Tutorial/result/test.tx");


    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("cant open the file: {:?}", error)
    };
}
