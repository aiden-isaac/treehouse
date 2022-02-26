use std::io::stdin;

fn main() {
    println!("Hello, what's your name?");
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    println!("Hello, {}", your_name)
}

// let declares a new variable
// String::new() is a value that got assigned
// std::io::stdin is the standard input system in rust
// Line 1 imports stdin so we don't have to keep typing that long line
// Line 6-8 is called function chaining, which basically passes the results of the first function 'till the last one
// Line 7's &mut makes the variable "borrowed" which allows the variable to be modified
// Line 8 "unwraps" the result object and it terminates the program with a specific message if an error occured
// Line 9 prints out the greeting, with the {} as the location of the variable, later included in the line after the string
