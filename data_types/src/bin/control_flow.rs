fn basic_with_cond(cond: bool) {
    if cond {
        println!("zero");
    } else {
        println!("non zero");
    }
}

fn basic_control_flow(number: i32) {
    if number < 5 {
        println!("smaller");
    } else if number == 5 {
        println!("equal");
    } else {
        println!("bigger");
    }
}

fn main() {
    let number = 3;
    basic_control_flow(number);

    basic_with_cond(true);
    basic_with_cond(false);
    // basic_with_cond(5); // retorna erro

    let cond: bool = false;
    let cond_stmt: i8 = if cond {5} else {8};
    println!("cond_stmt: {cond_stmt}");
}