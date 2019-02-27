/*
    Integers: ui, i8, u16, u32, i32, u64, i64, u128,
    i128 (number of bits they take in memory)
    Floats: F32, F64
    Boolean (bool)
    Characters (char)
    Tuples
    Arrays

*/
pub fn run() {
    // Default is "i132"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    //  Add explicit type

    let z: i64 = 454545454545;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression

    let is_greater: bool = 10 < 5;

    let a1 = 'a';

    println!("{:?}", (x,y,z , is_active, is_greater, a1 ) );    
}