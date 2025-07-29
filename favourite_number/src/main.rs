use std::io;

fn main() {
    loop {
        println!("What's my favourite number?");

        let fav: i32 = 44;

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = guess.trim().parse().expect("Please guess a number!");
        if guess == fav {
            println!("You guessed it!!");
            break;
        } else {
            println!("Try again!");
        }
    }
}
