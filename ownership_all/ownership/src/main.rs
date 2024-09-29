fn main() {
    {
        // 1. Each value in rust has a variable thats called its owner.
        //2. There can only be one owner at a time.
        //3. When the owner goes out of scope, the value will be dropped.

        //s is not valid here, its not yet declared
        let s: String = String::from("hello"); // s is valid from this point onwards

        //do stuff with s;
    } // the scope is now over so s is not longer valid!
}
