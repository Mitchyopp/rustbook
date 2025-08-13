use std::io;

fn main() {
    loop {
        println!("Please say something (type the word 'exit' to leave.)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let input = input.trim();

        let bad = ["test", "example"];

        if bad.iter().any(|&word| input.contains(word)) {
            println!("Please don't say that word here!");
        } else {
            println!("Thankyou for not saying anything wrong.");
        }

        if input == "exit" {
            break;
        }
    }
}
