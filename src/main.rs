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
        
    let known_visitor = visitor_list
        .iter()        
        .find(|visitor| visitor.name == name);
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You are not on the visitor list. Please leave."),
    }
}

// Iterators is a feature in Rust for manipulating data.
// Iterators are a bit of a catch-all feature. They can do a lot.
// When you're working with lots of data, iterators is the tool for it.
// Iterators are designed to function chain. It massages the data into what you can work with.
// Line 41 runs a closure called find. If the closure returns true, find returns the matching value.__rust_force_expr!
// Iterators are very fast and are great tools.