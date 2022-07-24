fn main() {
    let string = String::from("Hello, world!");
    let word = first_word(&string);
    let word = first_word(&string[..6]);
    let word = first_word(&string[..]);

    let my_string_literal = "hello.txt world";
    let word = first_word(my_string_literal);
    // string.clear();
    println!("first word is {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
