fn calculare_length(s: &String) -> usize {
    s.len()
}

fn change(s3: &String) {
    // s3.push_str(", la"); // não funciona pq a ref da linha 5 não é mut
    println!("s3: {}", s3);
}

fn mutable_change(s4: &mut String) {
    s4.push_str(" la");
    println!("s4: {}", s4);
}

fn main() {
    let s1 = String::from("hello");

    let len = calculare_length(&s1);

    println!("str {s1} w len {len}");

    change(&s1);

    let mut s1 = String::from("hello");

    mutable_change(&mut s1);
    mutable_change(&mut s1); // o escopo de mutable_change() acabou e é possível reutilizar
    {
        let _s2 = &s1; // também tem escopo definido
    }

    let _s3 = &mut s1;
    let _s4 = &mut s1;
    // println!("{_s3}, {_s4}"); // só pode fazer borrow de mut ref de s1 1x

    let _s5 = &s1; // ref fixa pra s1
    let _s6 = &mut s1; // ref mut pra s1
    // println!("{_s5}, {_s6}"); // dá ruim
}
