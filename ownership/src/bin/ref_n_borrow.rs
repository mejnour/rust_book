fn calculare_length(s: &String) -> usize {
    s.len()
}

fn change(s3: &String) {
    // s3.push_str(", la"); // não funciona pq a ref da linha 5 não é mut
}

fn main() {
    let s1 = String::from("hello");

    let len = calculare_length(&s1);

    println!("str {s1} w len {len}");

    change(&s1);
}
