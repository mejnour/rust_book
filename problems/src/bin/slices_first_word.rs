use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed");

    let word: String = input.trim().to_string();

    println!("{word}"); // trim só tira espacos do inicio e do fim

    let word_vec: Vec<&str> = input.trim().split_whitespace().collect();
    println!("{:?}", word_vec);
}