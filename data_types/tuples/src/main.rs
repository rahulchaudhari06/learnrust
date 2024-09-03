fn main() {
    let tup: (i32, f64, u8) = (530, 4.33233231, 4);

    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    println!("The value of x is: {x}");
    println!("The value of z is: {z}");
    //tuples have fized length once declared, they cannot grow or shrink in size.
    println!("{}", tup.0);
}
