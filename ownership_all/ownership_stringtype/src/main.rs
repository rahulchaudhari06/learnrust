fn main() {
    let mut s1 = String::from("Hellllloo!!!");
    //pushing "world" to this string...
    s1.push_str(" World!!!");
    let s2 = s1;

    println!("{s1}, world");
}
