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
    print!("The value of number is: {}", number);
}
