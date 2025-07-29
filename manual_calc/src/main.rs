use std::io;

const FIRST: i32 = 18;
const SECOND: i32 = 10;

fn main() {
    println!("Please change the variables.");
    println!("- + * /");

    let mut expression = String::new();
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read");

    if expression.trim() == "-" {
        subtract();
    } else if expression.trim() == "+" {
        add();
    } else if expression.trim() == "*" {
        multiply();
    } else if expression.trim() == "/" {
        divide();
    }


}

fn subtract() {
    let result = FIRST - SECOND;
    println!("{result}");
}
fn add() {
    let result = FIRST + SECOND;
    println!("{result}");
}

fn multiply() {
    let result = FIRST * SECOND;
    println!("{result}");
}
fn divide() {
    let result = FIRST / SECOND;
    println!("{result}");
}

// Proud i managed to make this work :D
