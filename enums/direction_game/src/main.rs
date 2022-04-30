// this allow unused variables
#![allow(dead_code)]
enum Direction {
    Up(i32, i32),
    Down(i32, i32),
    Left(i32, i32), 
    Right(i32, i32),
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}
impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_, _) => Keys::UpKey(String::from("Pressed w")),
            Direction::Down(_, _) => Keys::DownKey(String::from("Pressed s")),
            Direction::Left(_, _) => Keys::LeftKey(String::from("Pressed a")),
            Direction::Right(_, _) => Keys::RightKey(String::from("Pressed d")),

        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            // "ref s" is a refrense to value in "Keys::UpKey(String::from("Pressed w"))"
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}



fn main() {
    // Direction::Up(x, y)
    let up = Direction::Up(0, 1);
    let a = up.match_direction();
    println!("{:#?}", a);
    println!("{:?}", a.destruct());

}
