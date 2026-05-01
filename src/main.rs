use fake::{faker, Fake};
use rand::prelude::*;
use std::collections::BTreeMap;
use std::io::Write;

struct Name {
    first: String,
    last: String,
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("{} {}", self.first, self.last))
    }
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
        // input for command and parse arguments
        let input = input("> ").to_lowercase();
        let args: Vec<&str> = input.split_whitespace().collect();

        // handle command
        match args.first().copied() {
            Some("list") => {
                if class.is_empty() {
                    println!("Class is empty");
                    continue;
                }

                for (id, student) in &class {
                    println!("{:<5}{:<30}{:5}%",
                        id,
                        student.name,
                        student.grade,
                    );
                }
            }
            Some("info") => {
                // early exit
                let id: u32 = match args.get(1) {
                    None => {
                        println!("Missing argument");
                        continue;
                    }
                    Some(s) => match s.parse() {
                        Err(_) => {
                            println!("Invalid ID");
                            continue;
                        }
                        Ok(id) => id,
                    }
                };

                match class.get(&id) {
                    Some(student) => {
                        println!("{:<5}{:<30}{:5}%", id, student.name, student.grade);
                    }
                    None => println!("Student not found")
                }
            }
            /*Some("add") => {

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
            Some(unknown) => {
                println!("Unknown command: {unknown}");
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
