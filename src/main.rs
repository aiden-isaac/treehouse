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

// Closures are used a lot in Rust.
// For now (more detailed notes in thee future) think of closures as a function you define in place.
// Line 41's closure |visitor| visitor.name == name is the same as defining a function:
//     fn check_visitor_name (visitor: &Visitor, name: &String) -> bool {
//         return visitor.name == name;
//     }
// Closures can also capture data from the scope from which they are called.
//     You didn’t pass ‘name‘ to the closure; however, you were still able to use it anyway.