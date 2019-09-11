pub fn run() {
    let mut hello = String::from("BLOCKCHAIN ");

    // get length 
    println!("Length: {}", hello.len());
    
    // push a char
    //hello.push('q');
    // push string
    hello.push_str("network.");

    //capacity in include_bytes!("")
    println!("Capacity: {}", hello.capacity());

    // check if empty
    println!("Is Empty: {}", hello.is_empty());

    // contain some sub stringify!
    println!("Contains 'BLOCKCHAIN' {}", hello.contains("BLOCKCHAIN"));

    // replace
    println!("Replace: {}", hello.replace("BLOCKCHAIN", "5G"));

    // loop through string by whitespace, split the line
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity 
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // assertion testing
    assert_eq!(2, s.len());



    // println!("{}", hello);
}