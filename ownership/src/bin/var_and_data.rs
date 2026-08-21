fn main() {
    let x = 5;
    let y = x; // cópia de x
    println!("x: {x} | y: {y}");
    // inteiros tem o trait Copy
    // se um tipo tem o trait Drop, não pode ter o trait Copy
    // tudo que é trait Copy é alocado na stack
    // não faz diferença, neste caso, usar um método como clone()


    // s1 = (ptr, length e capacity) na stack
    // ptr aponta para a heap, onde o valor "hello" está nos indexes 0-4
    // len é o quanto da memória em bytes está em uso
    // cap é a quantidade total de memória recebida pelo alocador
    let s1 = String::from("hello");
    let s2 = s1; // copia (ptr, length e capacity). O espaco na heap é o mesmo de s1.

    // println!("s1: {s1}"); // err value borrowed here after move
    println!("s2: {s2}"); // s1 fica inválido depois de mover os valores de s1 -> s2

    let mut s3 = s2.clone(); // é criada cópia total, com dois espaços independentes na heap
    s3.push_str(" la");

    println!("s2: {s2} | s3: {s3}");
}