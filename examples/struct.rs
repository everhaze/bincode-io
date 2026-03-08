use std::io;
use binfile::{save, load};
use serde::{Serialize, Deserialize}; // requires serde = { version = "1.0.228", features = ["derive"] } in Cargo.toml

#[derive(Serialize, Deserialize)]
struct State {
    name: String,
    id: i32,
}

fn main() {
    println!("1: Save\n2: Load");
    let choice = ask();

    println!("Enter your path.");
    let path = ask();

    match choice.trim() {
        "1" => {
            println!("Enter your name.");
            let name = ask();

            println!("Enter your id");
            let sid = ask();
            let id: i32 = sid.trim().parse().unwrap();

            let mystate: State = State {
                name: name,
                id: id    
            };

            save(&path, mystate).unwrap(); 
            println!("Saved!");
        }
        "2" => {
            let mystate: State = load(&path).unwrap();
            println!("Loaded: {}, {}", mystate.name.trim(), mystate.id);
        }
        _ => {}
    }
}

fn ask() -> String {
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    answer
}
