#![warn(clippy::all, clippy::pedantic)]
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
    for i in 0..visitor_list.len() {
        if visitor_list[i] == name {
            allow_them_in = true;
        }
    }

    if allow_them_in {
        println!("Welcome to the Treehouse, {}", name);
    } else {
        println!("Sorry, you aren't on the list.");
    }
}

// Line 18 uses an enumerator to go through every value in the array
// This works but pedantic clippy will think otherwise:

// warning: the loop variable `i` is only used to index `visitor_list`
// --> src/main.rs:19:14
//    |
// 19 |     for i in 0..visitor_list.len() {
//    |              ^^^^^^^^^^^^^^^^^^^^^
//    |
// note: the lint level is defined here
// --> src/main.rs:1:9
//    |
// 1  | #![warn(clippy::all, clippy::pedantic)]
//    |         ^^^^^^^^^^^
// = note: `#[warn(clippy::needless_range_loop)]` implied by `#[warn(clippy::all)]`
// = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
// help: consider using an iterator
//    |
// 19 |     for <item> in &visitor_list {
//    |         ~~~~~~    ~~~~~~~~~~~~~