fn main() {
    let var_str = String::from("hello world");

    let hello = &var_str[0..5];
    let world = &var_str[6..11];

    println!("hello {hello} | world {world}");

    let len = var_str.len();

    let slice1 = &var_str[..5];
    let slice2 = &var_str[6..];
    let slice3 = &var_str[6..len];

    println!("slice1 {slice1} | slice2 {slice2} | slice3 {slice3}");

    let slice4 = &var_str[..];
    let slice5 = &var_str[0..len];

    println!("slice4 {slice4} | slice5 {slice5}");
}