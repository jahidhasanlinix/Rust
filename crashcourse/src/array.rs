// arrays are fixed list where elements are the same data types
use std::mem;
pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    // re-assigned values
    numbers[2]=20;

    println!("{:?}", numbers);

    // get single value
    println!("Single Value: {}", numbers[0]);

    // get array length
    println!("Array length: {}", numbers.len());

    // arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice from an arrays, 0..2 0 to 2
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

}