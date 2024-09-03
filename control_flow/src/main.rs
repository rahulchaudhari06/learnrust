fn main() {
    //If expressions
    let number = 3;
    if number < 5 {
        println!("True");
    } else {
        println!("false");
    }

    //using if in a let statement:
    let condition = true;
    let number = if condition { 5 } else { 6 };
    //this implies that if the condition is true the number will have value 5 and if false then will have value 6
    print!("{}", number);

    // fn main() {
    //   loop {
    //       println!("again!");
    //}
    //}
    // main();
    fn loooop() {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {count}");
    }
    loooop();

    //while loop-
    fn while_loop() {
        let mut number = 3;

        while number != 0 {
            println!("{number}!");

            number -= 1;
        }

        println!("LIFTOFF!!!");
    }
    while_loop();
}
