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
            VisitorAction::Accept => println!("Welcome to the treehouse, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("Note: {}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not let {} in!", self.name),
        }
    }
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("Fred", VisitorAction::Accept, 45),
        Visitor::new(
            "George",
            VisitorAction::AcceptWithNote {
                note: String::from("Milk is in the fridge"),
            },
            15,
        ),
        Visitor::new("Bob", VisitorAction::Refuse, 30),
    ];

    loop {
        println!("What's your name? (Leave empty and press enter to quit.)");
        let name = get_user_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0))
                }
            }
        }
    }
    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}

fn get_user_name() -> String {
    let mut user_name = String::new();

    stdin()
        .read_line(&mut user_name)
        .expect("Failed to read line");

    user_name.trim().to_lowercase()
}
