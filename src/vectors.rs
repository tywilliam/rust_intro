// Arrays - Fixed list where elements are the same data types
// Vectors are resizable arrays.
use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = [i32, 5] = vec![1,2,3,4];

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);


    // Pop off the last value
    numbers.pop();

    println!("{:?", numbers);

    // Get single val
    println!("Single Value: {}" , numbers[0]);

    // Get vector length
    println!("Vector Length {}": numbers.len());
    // Get array length 
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {}", numbers);
    
}