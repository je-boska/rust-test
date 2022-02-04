// Arrays - fixed list where elements are same data type

use std::mem;

pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    // Re-assign a value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single val
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Memory used: {}", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);
}
