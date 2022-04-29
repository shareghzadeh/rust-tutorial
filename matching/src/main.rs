fn main() {
    let x = 5;

    // each of the lines below is a condition
    // if x is 1 print "one" and so on ...
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("Somthing else"),
    }


    // more advance match

    let n = 15;

    match n {
        1 => println!("One"),
        // if it is 2 or 3 or ...
        2 | 3 | 5 |7 | 11 => println!("its prime"),
        // if it is in range of 13 to 19 include 19
        13 ..= 19 => println!("A teen"),
        _ => println!("its not special"),
    }


    let pair = (0, 2);

    match pair {
        (0, x) => println!("{}", x),
        (y, 0) => println!("{}", y),
        _      => println!("somthing went wrong"),
    }



    // if statement in match
    let pair2 = (4, 4);

    match pair2 {
        // (x, y) are represent of (4, 4) in abow
        (x, y) if x == y      => println!("Equal"),
        (x, y) if x + y == 0  => println!("both zero"),
        // "_" means igonre "y"
        (x, _) if x % 2 == 0  => println!("{} is even", x),
        _ => println!("no match"),

    }



    // pattern matching

    let p = 5;

    match p {
        // we bound "p = 5" in "n"
        // so we use "n@"
        // "n" is represent of "5"
        n @ 1 ..= 12 => println!("n: {}", n),
        n @ 13 ..= 19 => println!("n: {}", n),
        _ => println!("no match"),
    }


    // we can use match statment as expresion
    // in other word we can use match statement in let

    let p2 = 19;

    let res = match p2 {
        n @ 1 ..= 12 => n,
        n @ 13 ..= 19 => n,
        _ => 0,
    };
    println!("res: {}", res)
}
