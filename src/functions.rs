// Functions - Used to store blocks of code for re-use

pub fn run() {
    greeting("Hello", "Jane");
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}
