use std::io;
use binfile::{save, load};

fn main() {
    println!("1: Save\n2: Load");
    let choice = ask();

    println!("Enter your path.");
    let path = ask();

    match choice.trim() {
        "1" => {
            println!("Enter your name.");
            let name = ask();
            
            save(&path, &name).unwrap(); 
            println!("Saved: {}", &name);
        }
        "2" => {
            let name: String = load(&path).unwrap();
            println!("Loaded: {}", name.trim());
        }
        _ => {}
    }
}

fn ask() -> String {
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    answer
}