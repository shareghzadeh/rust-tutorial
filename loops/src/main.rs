fn main() {
    // infinite loop

    // loop {
    //     println!("infinite");
    // }

    // mut keyword make the variable mutable
    // it means you can change the value of variable
    let mut counter: u32 = 0;

    loop {
        // add +1 to counter for each loop
        counter += 1;
        println!("{}-finite", counter);

        // if counter == 10 it will break the loop
        // with out this if statment we would have
        // infinite loop
        if counter >= 10 {
            break;
        }
    }


    // labeling loop

    'a: loop {
        println!("loop a");
        'b: loop {
            println!("loop b");
            'c: loop {
                println!("loop c");
                break 'b
            }
        }
    }

    
}
