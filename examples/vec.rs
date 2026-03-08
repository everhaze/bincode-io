use std::io;
use bincode_io::{save, load};

fn main() {
    println!("1: Save\n2: Load");
    let choice = ask();

    println!("Enter your path.");
    let path = ask();

    match choice.trim() {
        "1" => {
            println!("Enter your name.");
            let name = ask();
            let tname = name.trim();
            let vec = [&tname, &tname, &tname];

            save(&path, vec).unwrap(); 
            println!("Saved!");
        }
        "2" => {
            let myvec: [String; 3] = load(&path).unwrap();
            println!("Loaded: {:?}", myvec);
        }
        _ => {}
    }
}

fn ask() -> String {
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    answer
}
