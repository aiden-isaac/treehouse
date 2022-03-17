use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }
    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!(
                "Welcome to the tree 
          house, {}",
                self.name
            ),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!(
                "{} is now a 
          probationary member",
                self.name
            ),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
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
    let mut visitor_list = vec![
        Visitor::new("Bert", VisitorAction::Accept, 45),
        Visitor::new(
            "Steve",
            VisitorAction::AcceptWithNote {
                note: String::from("Lactose-free milk is in the fridge"),
            },
            15,
        ),
        Visitor::new("Fred", VisitorAction::Refuse, 30),
    ];
    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                }
            }
        }
    }
    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}

// In this treehouse, the bouncer needs more detail on how to treat different visitors with more detail.
// We need to store an action associated with a visitor:
//     admit them,
//     admit them with a note,
//     refuse entry,
//     or mark them as probationary members.
// We also need to store the visitors age and forbid them from drinking if they're under 21.

// Enums (short for enumerations) are like structures but with one difference.
// Use a struct when you want one thing AND another thing.
// Use an enum when you want one thing OR another thing.
// In line 15, i8 is a variable type. It means 8-it signed integer, meaning it can hold values from -128 to 127.
// You can access the members of an enumeration with the :: operator (Enumeration::Member).
// You assign values to an enumerators the same way you assign values to structs.
// Enumerations can be a complex type—comparing with an if statement might not make sense if you also want to consider wrapped values.
// Instead, you need to use pattern matching.
// Pattern matching serves two basic purposes: it checks to see if a condition is true and runs the associated code,
//     and it can extract fields from complicated types (such as your enum’s note).
// In line 27, it greets the visitor with different greetings based on the action given.