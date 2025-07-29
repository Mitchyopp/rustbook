use std::io;

fn main() {
    println!("What is your name?");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    if name.trim() == "Mitch" {
        println!("Your special!");
    } else {
        println!("Your name is: {name}");
    }
}
