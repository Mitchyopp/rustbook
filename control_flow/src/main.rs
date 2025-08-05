use std::io;

fn main() {

    println!("Enter your name: ");
    let mut name = String::new();

    io::stdin() 
        .read_line(&mut name)
        .expect("Expected readline");

   if name.trim() == "Mitch" {

                println!("W");
                let password = "cool";
                let mut user_attempts = 0;

                loop {
                    println!("Please enter the password to continue.");
                    let mut password_guess = String::new();

                    io::stdin()
                        .read_line(&mut password_guess)
                        .expect("Failed to read password");

                    if password.trim() == password_guess.trim() {
                        let namee = name.trim();
                        println!("You got the password correct, welcome back {namee}.");
                        break;
                    } else {
                        user_attempts += 1;
                        println!("Password failed. {user_attempts} of 3 attempts.");

                        if user_attempts >= 3 {
                            println!("Passed the attempt limit");
                            break;
                        }
                    }
                }
        } else {
            println!("L");
    }
}
