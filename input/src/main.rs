use std::io;

fn main() {
    println!("Please enter your name");
    use std::io;
    let mut name = String::new();
    io::stdin()
      .read_line(&mut name)
      .expect("Failed to read line");
    println!("{name}");
}
