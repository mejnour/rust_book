// s é criada e vive durante o escopo de dangle()
// dangle retorna ref de s
// acaba dangle() e s também acaba
// ref de s fica órfã
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// solução: retornar s pra que o ownership também seja devolvido
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn main() {
    // let ref_to_ntn = dangle();
    let ref_to_s = no_dangle();
    println!("{ref_to_s}");
}