/*
Variables hold primitive data or references to data
Variables are immutable by default
Rust is a block-scoped language
*/

pub fn run() {
    let name = "Jon Eirik";
    let mut age = 35;
    println!("My name is {} and I am {}", name, age);
    age = 36;
    println!("My name is {} and I will be {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, my_age) = ("Jon Eirik", 35);

    println!("{} is {}", my_name, my_age);
}
