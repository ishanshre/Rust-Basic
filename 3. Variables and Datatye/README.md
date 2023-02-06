# Variables
    - Variables are storage identifier that rust can manipulate its values
    - When variables is created a memory space is allocated
    - Size of space depends upon the datatype
    - We create variables using `let` keyword
        ```
        let x:i64 = 15;
        ```
    - We must tell the rust compiler the datatype of variable. In above example i64 is the datatype
    - In rust, variables are immutable by default.
    - To make variables mutable we must explictly define it as mutable using `mut` keyword
        ```
        let mut x:i64 = 15;
        ```


# Data Type

## Scaler Data Type
    - Interger
      - Signed 
        - i8, i16, i32, i64, i256
        - Calculating the range of the data type is given by : -2^(n-1) to (2^(n-1)-1)
      - Unsigned Integer
        - u8,u16,u32,u64

        ```
        println!("The range for i64 is {min} to {max}", min=std::i64::MIN, max=std::i64::MAX);
        ```

    - Float:
      - f32, f64
        
        ```
        println!("For f32 datatype the range is {min} to {max}", min=std::f32::MIN, max=std::f32::MAX);
        ```