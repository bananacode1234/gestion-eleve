use fake::{faker, Fake};
use rand::prelude::*;
use std::collections::BTreeMap;
use std::io::Write;

struct Name {
    first: String,
    last: String,
}

struct Student {
    name: Name,
    grade: u8,
}

fn main() {
    let mut class: BTreeMap<u32, Student> = BTreeMap::new();
    let mut rng = rand::rng();

    // generate placeholder data
    for i in 1..20 {
        class.insert(i, Student {
            name: Name {
                first: faker::name::en::FirstName().fake(),
                last: faker::name::en::LastName().fake(),
            },
            grade: rng.random_range(0..=100),
        });
    }

    // main loop
    loop {
        // input for command
        let command = input("> ");

        // handle command
        match command.split_whitespace().next() {
            Some("list") => {
                for (id, student) in &class {
                    println!("#{}: {} {}: {}%",
                             id,
                             student.name.first,
                             student.name.last,
                             student.grade
                    );
                }
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
                // don't error if there is no command
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
