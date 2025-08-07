fn main(){
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break
            count * 2
        }
    };
    println!("Result: {result}");

    let mut number = 18;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("You escaped!");

    let mut goal = 1;
    while goal <= 69 {
        println!("KEEP GOING {goal}");
        goal += 1;
        clean_line();
    }

    for what in (1..4).rev() {
        println!("{what}");
    }
    println!("lol");

    for what in 1..4 {
        println!("{what}");
    }
    println!("lol");
}

fn clean_line() {
    println!("------------------------");
}
