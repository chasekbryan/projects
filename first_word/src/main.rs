fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // 'word' still has the value '5' here, but 's' no longer has any content
    // that we could meaningfully use with the value '5', so 'word' is now
    // totally invalid!
    println!("the first word is: {word}"); // outputs 5
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}