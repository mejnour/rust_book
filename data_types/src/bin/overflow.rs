fn main() {
    let mut a: u8 = 0;
    let mut overflowd:bool = false;

    loop {
        a = a + 1;

        if a < 10 && a > 250 {
            println!("{a}");
        }

        if a > 100 {
            overflowd = true;
        }

        if a < 10 && overflowd {
            break;
        }
    }
}
