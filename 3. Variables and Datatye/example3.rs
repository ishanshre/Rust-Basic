fn main() {
    // for bool data type it is either true or false
    
    let x :i32 = 13;
    let not_equal :bool = 13 != x;
    println!("The value of condition 13 != 13 is {}", not_equal);


    // ----------------------------
    // -----Character--------------
    // ----------------------------
    let c1: char = 'a';
    let c2: char = 'b';
    let c3: char = '1';
    let c4: char = '\u{288A}';
    println!("Printing characters---------------");
    println!("c1= {}, c2 = {} , c3 = {} , c4 {}", c1, c2, c3, c4);

}