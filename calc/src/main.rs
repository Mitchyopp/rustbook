use std::io;

fn main() {
    loop {
        //
        println!("Welcome to my calculator, Please enter your first number");

        let mut first_calc = String::new();

        io::stdin()
            .read_line(&mut first_calc)
            .expect("Failed to read line");

        println!("What type of expression are you doing? - * + /");

        let mut expression = String::new();

        io::stdin()
            .read_line(&mut expression)
            .expect("Failed to read expression");

        println!("Please enter the second number!");
        let mut second_calc = String::new();

        io::stdin()
            .read_line(&mut second_calc)
            .expect("Failed to read second calc");


        let first_calc: i32 = first_calc.trim().parse().expect("Number please!");
        let second_calc: i32 = second_calc.trim().parse().expect("Number please!");

        println!("Your result:");

        if expression.trim() == "-" {
            let result = first_calc - second_calc;
            println!("{result}");
            break;
        } else if expression.trim() == "*" {
            let result = first_calc * second_calc;
            println!("{result}");
            break;
        } else if expression.trim() == "+" {
            let result = first_calc + second_calc;
            println!("{result}");
            break;
        } else if expression.trim() == "/" {
            let result = first_calc / second_calc;
            println!("{result}");
            break;
        } else {
            println!("Please enter a valid expression.");
        }
    }
}
