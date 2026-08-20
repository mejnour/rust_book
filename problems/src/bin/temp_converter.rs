use std::io;

fn f_to_c(f_temp: i32) {
    let c_temp: i32 = ((f_temp - 32) * 5)/9;
    println!("Temperatura convertida: {c_temp}C");
}

fn c_to_f(c_temp: i32) {
    let f_temp: i32 = ((c_temp * 9) / 5) + 32;
    println!("Temperatura convertida: {f_temp}F");
}

fn read_temp_input(unit: char) -> i32 {
    println!("Que valor de tempartura deseja converter?");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed");

    let temp = temp
        .trim()
        .parse()
        .expect("NaN");

    if unit == 'F' {
        println!("Input de Temperatura: {temp}F");
    } else if unit == 'C' {
        println!("Input de Temperatura: {temp}C");
    } else {
        println!("Unidade inválida");
    }

    temp
}

fn read_option() -> i8 {
    println!("Que conversão deseja fazer?");
    println!("1 - Celsius para Fahrenheit;");
    println!("2 - Fahrenheit para Celsius;");

    let mut conv = String::new();
    io::stdin()
        .read_line(&mut conv)
        .expect("Failed");

    let conv: i8 = conv
        .trim()
        .parse()
        .expect("NaN");

    println!("Opção: {conv}");

    conv
}

fn main() {
    println!("Conversor de Temperatura");

    let option: i8 = read_option();

    if option == 1 {
        c_to_f(read_temp_input('C'));
    }

    if option == 2 {
        f_to_c(read_temp_input('F'));
    }
}
