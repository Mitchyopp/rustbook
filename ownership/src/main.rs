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


    let mut first_name = String::from("Mitch");
    first_name.push_str("opp");
    println!("{first_name}");

    let s1 = String::from("hello");
    println!("{s1}");
    let s2 = s1.clone();
    println!("{s2}");
    let s2 = "Changed";
    println!("{s2}");
}
