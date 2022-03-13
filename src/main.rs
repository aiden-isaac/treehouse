use std::io::stdin;

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    let visitor_list = ["bert", "steve", "fred"];

    println!("Hello, what's your name?");
    let name = what_is_your_name();

    let mut allow_them_in = false;
    for visitor in &visitor_list {
        if visitor == &name {
            allow_them_in = true;
        }
    }

    if allow_them_in {
        println!("Welcome to the Treehouse, {}", name);
    } else {
        println!("Sorry, you aren't on the list.");
    }
}

// Rust's simplest list type is an array
// There are two rules to the values in an array:
//     1. The values must be of the same type
//     2. The array cannot change in size
// Line 12 shows the declaration of an array with string literals (&str types)
// If you wanted to declare the array type, the syntax would be:
//     let visitor_list : [&str;3] = ...
// Index numbering is similar in Python, it counts from 0
// Rust has two types of strings,
//     str; string literals entered in source code, generally unchanging
//     String; dynamic because you can store location, length and capacity
