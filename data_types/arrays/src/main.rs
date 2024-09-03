use std::io;

fn main() {
    let arr = [10, 20, 30, 40, 50];
    println!("Please enter the posn of element!");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("index entered was not an number!");

    let element = arr[index - 1];
    println!("The value of the element at index {index} is: {element}");
}
