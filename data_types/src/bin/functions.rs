fn plus_one(x: i8) -> i8 {
    x + 1
}

fn plus_one_i32(x: i32) -> i32 {
    x + 1
}

fn five() -> i8 {
    5
}

fn main() {
    println!("Hello!");

    another_function();
    second_function(5);
    print_labeled_measurement(5, 'L');

    let y = {
        // o bloco de código dentro de {} é uma expressão
        // que é executada e tem seu valor associado à
        // declaração let y = {...};
        let z = 4;
        4 + 10 + z // sem ; no final, senão vira declaração
    };

    println!("y: {y}");

    let five = five();
    println!("five: {five}");

    println!("six: {0}", plus_one(five));
    println!("six: {0}", plus_one_i32(five.into())); // cast?
}

fn another_function() { // snake case
    println!("another_function");
}

fn second_function(x: i32) {
    println!("x: {x}");
}

fn print_labeled_measurement(value: i32, unit: char) {
    println!("{value}{unit}");
}