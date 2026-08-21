fn makes_copy(some_int: i32) {
    println!("{some_int}");
}

fn takes_ownership(some_str: String) {
    println!("{some_str}");
}

fn main() {
    let s = String::from("hello"); // trait drop
    takes_ownership(s);
    // println!("{s}"); err value borrowed here after move
    // s foi movido pra takes_ownership()

    let x = 5;
    makes_copy(x);
    println!("{x}"); // tá na stack, funciona de boa
}