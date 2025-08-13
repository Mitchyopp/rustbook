fn read(y: bool) {
    if y {
        println!("Y is true!");
    }
}

fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn main() {
    let x = true;
    read(x);

    let a = 5;
    let mut b = a;
    b += 1;
    println!("{a} {b}");

    let my_str = String::from("hello");
    print_length(&my_str);
    println!("{my_str}");
}
