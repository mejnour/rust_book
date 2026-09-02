fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[(i + 1)..]
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("hello world");
    let f_str = first_word(&s);
    let s_str = second_word(&s);
    println!("f_str {f_str}");
    println!("s_str {s_str}");
}