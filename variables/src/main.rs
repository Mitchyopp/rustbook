use std::io;

fn main() {
    println!("Welcome");
    let name = "Mitch";
    println!("Your name is {name}");

    println!("Please enter your new name");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Your new name is: {name}");
}
