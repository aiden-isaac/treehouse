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

// Structs are similar to classes in other languages
// It creates a whole new variable type, such as String
// Creating a structure is easy, it's shown in Line 3-6
// The impl in Line 8 is a function that implements methods in the structure and helps initialize it
// A constructor (Line 10) is a function custom made to create instances of the structure easier and more customizable
// Self in Line 9 is a shorthand way to say "Visitor"
// self refers to the instance made in the constructor
// Line 16 is a member function (a method). It accepts self as a parameter, which is
//     automatically passed into the function when you reference an instance of the struct (my_visitor.greet_visitor())
//     with the contents of that specific structure instance.
