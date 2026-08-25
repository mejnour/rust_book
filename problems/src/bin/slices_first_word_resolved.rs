fn first_word_1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let qualquer_str: String = String::from("teste teste");
    let index = first_word_1(&qualquer_str);
    println!("index: {index}");
    println!("palavra: {}", &qualquer_str[0..index]);
}