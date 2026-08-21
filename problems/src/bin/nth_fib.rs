use std::io;

fn calc_fib_elem_by_pos(pos: i32) {
    if pos == 1 {
        println!("pos 1 is elem 0");
    } else if pos == 2 {q
        println!("pos 2 is elem 1");
    } else if pos > 2 {
        let mut a:u128 = 0;
        let mut b:u128 = 1;
        for _ in 1..(pos-1) {
            let temp = a + b;
            a = b;
            b = temp;
        }
        println!("pos {pos} is elem {b}");
    } else {
        println!("Posição inválida");
    }
}

fn read_position() -> i32 {
    println!("Posição do elemento:");
    let mut posicao = String::new();
    io::stdin()
        .read_line(&mut posicao)
        .expect("Failed");

    let posicao: i32 = posicao
        .trim()
        .parse()
        .expect("NaN");

    println!("Retornar o elemento da sequência fibonacci na posição {posicao}");

    posicao
}

fn main() {
    println!("Retorna o nth elemento de Fibonacci");
    calc_fib_elem_by_pos(read_position());
}