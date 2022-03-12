use std::io::stdin;

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}
fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();
    println!("{:?}", name);
}

// Line 13 adds a debugging parameter which helps show us the exact value of the variable
// The output comes out as "test\n", and \n is the ENTER indent. This is called a carriage return.
// Rust includes a trim function to remove these characters. Which is a good idea since you need the variables to match with what you have on mind ("Name" =/= "Name\n")
// It's also a good idea to always make the output lowercase so then the letters will always match ("nAmE" =/= "name")
