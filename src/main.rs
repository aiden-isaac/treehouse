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
    if name == "bert" || name == "steve" {
        println!("Welcome {}", name);
    } else {
        println!("Sorry, you aren't on the list, {}", name)
    }
}

// Line 13 shows boolean logic, || means "or", && means "and", ! means not