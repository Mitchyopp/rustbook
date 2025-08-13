# Why ownership exists

Rust does not have a garbage collector, instead you use ownership at compile time, it makes sure every chunk of is cleaned up exactly once.
It does this without a garbage collector which means that programs can run faster and with less runtime memory overhead.

# 3 rules

Each value in rust has a single owner

Example:
`let s = String::from("hello");` s owns that string.

When the owner goes out of scope the value is dropped (freed)

Example:
```rust
{
    let s = String::from("hi");
} // s goes out of scope -> memory gets freed
```


Transferring ownership moves the value not copies it (unless its copy)

Example:

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{s1}"); // Error, value was moved.
```

# What is happening in memory

Doing this with a string because it's easier

A string has:

A pointer to its data on the heap
A length
A capacity

when you do:

`let s1 = String::from("hello");`
s1 is on the stack pointing to the actual text "hello" on the heap.

so when you do:

`let s2 = s1;`

rust copies the pointer, length and capacity into s2 but does NOT copy the heap data (for performance)
to prevent both s1 and s2 from trying to free the same heap data later rust invalidates s1.
