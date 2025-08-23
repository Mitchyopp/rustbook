fn main() {
    let x = String::from("Hello");
    let y = x.len();
    println!("{y}");

    let a = 3;
    let b = 1 + 2;
    assert_eq!(a, b); // This is used in testing to make sure 2 values are the same otherwise rust
                      // panics.

    let v = vec![1, 2, 3];
    println!("{v:?}");
}
