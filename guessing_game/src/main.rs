use std::io;
use std::cmp::Ordering;  //cmp comparing two values
use rand::Rng;  // rng trait define method, random no generators impl

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);  // rand::thread_rng() give particular random no generator
    // gen_range takes two no as arg and gen a rand no. 100 rqst

    println!("The secret number is: {}", secret_number);

    println!("Enter your guess number: ");

    let mut guess = String::new();  // mutable variable, bound to new, empty instance of string
    io::stdin().read_line(&mut guess).expect("Failed to read line"); // calls read_line, std ip handle to get ip from the user, mut guess arg passing.  & means arg is a reference.
// .expect syntax introduce a newline, or breakup into long line

    let guess: u64 = guess.trim().parse()    // trim eliminates \n new line eg: 5\n > only 5 no /n. parse is string, variety of no types. 
        .expect("Please type a number!");  // : colon after guess tells the variables type

// Handling invalid ip
// let guess: u64 = guess.trim().parse() {
    // ok(num) => num,
    // Err(_) => continue,
// };


    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win"),
    }
    println!("You guessed: {}", guess)  // {} is a placeholder.
} 

