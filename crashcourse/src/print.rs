pub fn run() {
    println!("My name is Jahid Hasan");

    // Basic formatting
    println!("Age: {}", 25);

    println!("{} is from {}.", "Jahid", "Bangladesh");

    //Positional format_args!("")
    println!("{0} is from {1} and {0} likes to {2}.", "Jahid", "Bangladesh", "code");

    //Named format_args
    println!("{name} like to play {activity}.", name="Jahid", activity="Cricket");

    //place holder trait Name , trait for the binary is :B, this will give binary for all
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10);

    // debug trait, ? mark we can put multiple value
    println!("{:?}", (12, true, "thanks"));

    // Basic math 
    println!("10+10={}", 10+10)
    
}