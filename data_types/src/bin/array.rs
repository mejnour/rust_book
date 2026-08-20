use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);
    println!("a: [{0}, {1}, {2}, {3}, {4}]", a[0], a[1], a[2], a[3], a[4]);

    let b: [i32; 3] = [1, 2, 3];
    println!("b: {:?}", b);

    let c = [5; 3];
    println!("c: {:#?}", c);

    println!("Index:");
    let mut index = String::new();
    io::stdin()                     // se colocar index out of bound dá ruim em runtime
        .read_line(&mut index)
        .expect("Failed");

    let index: usize = index
        .trim()
        .parse()
        .expect("NaN");

    println!("array b. index: {index}, elem: {0}", b[index]);
}