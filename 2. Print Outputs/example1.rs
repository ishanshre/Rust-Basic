fn main() {

    // ! -> at end of println, ! means marcr.
    // macro in rust is also knows for meta programming
    // It means the piece of code can be subsituted by other code before execution
    print!("Printing in a line");
    print!("Will Continue in a same line");
    println!(); // print a next line
    print!("This is in a line");

    println!("This prints in newline"); // println!() prints a statement and add a new line
    println!("This prints in newline");
    
    // Curly brases inside the statement will be replaced with arguments
    // {} also known as placeholder
    println!("This is a line and curly bracket is relplaced with arguments --> {}", "This is a argument");

    // integer can be printed only with placeholer
    println!("The sum of 25 + 10 = {}", 25+10);

    // We can also specify the positional argument in curly braces
    // index for argument start with 0
    println!("This is {1}. This is a {0}", "car","apple");

    // we can also assign named argument instead of positional argument
    println!("This is line for named argument {name} {email}", name="Ishan", email="sihan@asd.com");

    // different forms can be displayed by sepcifying the format character
    // specify format character after ":" inside the curly braces

    println!("Base 10: {}", 10);
    println!("Base 2 (binary): {:b}", 10);
    println!("Base 8 (octal): {:o}", 10);
    println!("Base 16 (hexadecimal): {:x}", 10);
    println!("Base 16 (HEXADECIMAL): {:X}", 10);

    // in this format we add 5 whitespace before the number with format >5 after ":"
    println!("{number:>5}", number=1);
    println!("{number:>9}", number=10);

    // replace 2 whitespace with 1 resulting 411
    println!("{number:1<3}", number=4);

    // replace 2 whitespace before number with 2 resulting 224
    println!("{number:2>3}", number=4);
    //

    println!("{number:1>width$}", number=43, width=11);
}