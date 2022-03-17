use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    let visitor_list = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];

    println!("Hello, what's your name?");
    let name = what_is_your_name();

    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You are not on the visitor list. Please leave."),
    }
}

// The find closure returns an Option type.
// Options either contain a value or they don’t.
// Rust options are an enumeration.
// Options have two possible values: Some(x) and None.
// There are many ways to interact with and extract data from an option.
// In this program, line 42 uses match with the option data.
// match some(visitor) checks to see if the option has data and makes the contents of the option available to the code.
// The fat arrow in line 43 and 44 (=>) denotes the code to execute for this match.
