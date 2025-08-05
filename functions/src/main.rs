fn main() {
    cool();
    arg(1000);
    wrong("Yeah so, turns out i don't really know that much about functions, lol.");

    let y = {
        let x = 10;
        x + 1
    };
    println!("{y}");
}

fn cool() {
    println!("Bold of you to assume i don't know how a function works");
}

fn wrong(welp: &str) {
    println!("{welp}");
}

fn arg(x: i32) {
    println!("{x}")
}
