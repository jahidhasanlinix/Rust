// tuple grp together values of different type, max 12 elements
pub fn run() {
    let person: (&str, &str, i8) = ("Jahid", "Bangladesh", 25);

    println!("{} is from {} and is {}.", person.0,person.1,person.2);

}