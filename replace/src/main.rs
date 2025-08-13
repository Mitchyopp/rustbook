use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter your scentence.");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let output = input.replace(" ", "");
    println!("{output}");
}
