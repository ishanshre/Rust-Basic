fn main() {
    // ------Variables in Rust ---------//
    // declare a variable in rust using let keyword
    // varaible in rust is immutable by default
    // to make it mutable we must explictly tell rust using mut keyword
    let x:i64 = 15;
    println!("The value of variable x = {}", x);
    
    // creating a mutable variable
    let mut y:i32 = 70;
    println!("Value of y before changing: {}", y);
    y = 100;
    println!("Value of y after changing: {}", y);
    
}