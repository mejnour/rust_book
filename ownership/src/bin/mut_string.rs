fn main() {
    let mut s = String::from("hello"); // alocado na heap
    s.push_str(", world!");
    println!("{s}");

    let mut s2 = "teste"; // alocado na stack
    // s2.push_str(", la"); // err method not found in `&str`
    println!("{s2}");
}
