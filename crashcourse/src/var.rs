pub fn run() {
    let name = "Jahid";
    let mut age = 25;
    println!("My name is {} and I am {}.", name, age);
    age = 26;
    println!("My name is {} and I am {}.", name, age);

    // define contant 
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple var 
    let (my_name, my_age) = ("Jahid", 25);
    println!("{} is {}.", my_name, my_age);
}