fn main() {
    // addition
    let sum = 5 + 10;
    println!("sum: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product: {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("quotient: {quotient}");
    println!("truncated: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {remainder}");

    // power
    let power_of = 2_i32.pow(2);
    println!("potenciacao int {}", power_of);

    // power float
    let power_of = 2.0_f32.powf(2.0);
    println!("potenciacao float {}", power_of);

    // raiz quadrada
    let sqrt_ex = 16_f32.sqrt(); // só existe sqrt() para tipos float, f32 e f64
    println!("raiz quadrada de 16.0 é {}", sqrt_ex);
}