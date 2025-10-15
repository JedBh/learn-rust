use std::io::Bytes;

fn main() {
    let _y = "hello";
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);
    // === Ownership rules ===
    // 1. Each value in Rust has a variable that's called it's owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    {
        let s1: String = String::from("Hello");
        let s2: String = s1;

        println!("{}", s2);
    }

    {
        let s1: String = String::from("hello world");
        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    {
        let mut s1: String = String::from("Hello imma");
        change(&mut s1);

        println!("mutated string: {}", s1)
    }

    // The Rules of References
    // 1. At any given time, you can have either one mutable reference
    // or any number of immutable references.
    //
    // 2. References must always be valid.

    // === Slice Type ===
    {
        let mut s: String = String::from("Hello I am Jedidiah Ben Hod");
        let s2 = "hello world";
        let hello = &s[..5];
        let world = &s[6..];

        let word = first_word(&s);

        println!("Hello: '{}', world: '{}'", hello, world);
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn change(some_string: &mut String) {
    some_string.push_str(", I love you.");
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();

    length
}
