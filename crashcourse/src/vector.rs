// vectors are resizeable array
use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    // re-assigned values
    numbers[2]=20;

    // add on a vectors
    numbers.push(5);
    numbers.push(6);

    // pop off last values
    numbers.pop();

    println!("{:?}", numbers);

    // get single value
    println!("Single Value: {}", numbers[0]);

    // get vector length
    println!("Vector length: {}", numbers.len());

    // vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice from an arrays, 0..2 0 to 2
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    //loop through vec![]
    for x in numbers.iter() {
        println!("Numbers: {}", x);
    }

    // loop and mut value
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);

}