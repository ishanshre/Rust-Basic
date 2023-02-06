fn main() {
    // std means standard package in rust which is globally enabled
    // i8 is the datatype
    // MIN and MAX are the modules in the std package
    println!("Calculating the range of signed integer");
    println!("The range for storing integer in i8 datatype of 8 bit integer is {min} to {max}", min=std::i8::MIN, max=std::i8::MAX);
    println!("The range for i16 is {min} to {max}", min=std::i16::MIN, max=std::i16::MAX);
    println!("The range for i32 is {min} to {max}", min=std::i32::MIN, max=std::i32::MAX);
    println!("The range for i64 is {min} to {max}", min=std::i64::MIN, max=std::i64::MAX);
    println!("The range for i128 is {min} to {max}", min=std::i128::MIN, max=std::i128::MAX);

    println!();
    println!("Calculating the range of unsigned integer");
    // unsigned integer
    println!("The range for storing integer in u8 datatype of 8 bit integer is {min} to {max}", min=std::u8::MIN, max=std::i8::MAX);
    println!("The range for u16 is {min} to {max}", min=std::u16::MIN, max=std::i16::MAX);
    println!("The range for u32 is {min} to {max}", min=std::u32::MIN, max=std::i32::MAX);
    println!("The range for u64 is {min} to {max}", min=std::u64::MIN, max=std::i64::MAX);
    println!("The range for u128 is {min} to {max}", min=std::u128::MIN, max=std::i128::MAX);

    println!("Calculating the range of floating point");
    println!("For f32 datatype the range is {min} to {max}", min=std::f32::MIN, max=std::f32::MAX);
    println!("for f64 the range is {min} to {max}", min=std::f64::MIN, max=std::f64::MAX);
}