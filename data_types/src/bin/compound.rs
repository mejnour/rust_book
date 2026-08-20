fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (first, second, third) = tup;
    println!("first: {first}, second: {second}, third: {third}");

    let tup2 = (500, 6.4, 1);
    println!("first: {0}, second: {1}, third: {2}", tup2.0, tup2.1, tup2.2);
}