# Print in rust
    - Printing in rust is handled by series of macros defined in module std::fmt
    - example:
        - ```format!``` : write formatted text to string
        - print! : It is same as format! but the text is printed to the console (io::stdout).
        - println! : it is same as print! but a newline is appended.
        - eprint!: It same as print! but the text is printed to the standard error (io::stderr).
        - eprintln! : It same as eprint! but a newline is appended.

    - ! at last of println or any syntax represents macro.
      - macro refers a metaprogramming in rust
      - In simple term, a piece of code subsituted by other code before the execution

    - When printing we cannot print integer like other programming. We must use placeholder in rust
      - Placeholder --> {}
        ```
        println!("The value of constant is {}", 232);
        ```
    
      - We can use place holder to replace with the argument
      - Other examples we use positional argument, named argument and the format specifier
        ```
        println!("This is {1}. This is a {0}", "car","apple");
        println!("This is line for named argument {name} {email}", name="Ishan", email="sihan@asd.com");
        ```

    - Different forms can be displayed by sepcifying the format character specify format character after ":" inside the curly braces
      - for example 
        - {:b} represents binary format
        - {:o} represents octal format
        - {:x} represents hexadecimal format
        - {:X} represents hexadecimal format in uppercase
        
      ```
      println!("Base 10: {}", 10);
      println!("Base 2 (binary): {:b}", 10);
      println!("Base 8 (octal): {:o}", 10);
      println!("Base 16 (hexadecimal): {:x}", 10);
      println!("Base 16 (HEXADECIMAL): {:X}", 10);
      ```
    
    - We can add whitespace before we print
      ```
      println!("{number:>5}", number=1);
      println!("{number:>9}", number=10);
      ```

    - 