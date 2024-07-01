fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello: {hello}");
    println!("word: {world}");

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    println!("slice: {slice}");
    let slice = &s[3..];
    println!("slice: {slice}");

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    println!("slice: {slice}");
    let slice = &s[..];
    println!("slice: {slice}");

    let s = String::from("hello world");

    let word = first_word(&s);

    // s.clear();

    println!("the first word is: {}", word);

    // others
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers;
    assert_eq!(slice, &[1, 2, 3, 4, 5]);
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
