fn main() {
    // If expressions
    let number = 3;
    if number < 5 {
        println!("True");
    } else {
        println!("False");
    }

    // Using if in a let statement:
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // This implies that if the condition is true, the number will have value 5; if false, then it will have value 6
    println!("{}", number);

    // Function that demonstrates loop with labels
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
                remaining -= 1;

                if count == 2 {
                    break 'counting_up;
                }

                count += 1;
            }
            println!("End count = {count}");
        }
    }
    loooop();

    // While loop demonstration
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
