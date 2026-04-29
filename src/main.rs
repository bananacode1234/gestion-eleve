use fake::{Fake, faker};
use rand::prelude::*;
use std::io::Write;

struct Name {
    first: String,
    last: String,
}

struct Student {
    name: Name,
    grade: f64,
}

fn main() {
    let mut class: Vec<Student> = Vec::new();
    let mut rng = rand::rng();

    for _ in 1..20 {
        class.push(Student {
            name: Name {
                first: faker::name::en::FirstName().fake(),
                last: faker::name::en::LastName().fake(),
            },
            grade: rng.random_range(0.0..=100.0),
        });
    }

    // main loop
    loop {
        // input for command
        let command = input("> ");

        // handle command
        match command.split_whitespace().next() {
            Some("list") => {
                println!("{}", class
                    .iter()
                    .map(|s| format!("{} {}", s.name.first, s.name.last))
                    .collect::<Vec<_>>()
                    .join(", ")
                );
            }
            /*Some("info") => {

            }
            Some("add") => {

            }
            Some("remove") => {

            }*/
            Some("exit") => {
                break;
            }
            None => {
                continue;
            }
            _ => {
                println!("Unknown command: {}", command);
                println!("Please try again");
                continue;
            }
        }
    }
}

// helper input function
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().to_owned()
}
