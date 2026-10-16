use crate::garden::vegetables::Aspargus;

pub mod garden;

fn main() {
    let plant = Aspargus {};
    println!("I'm a growing {plant:?}!");
}
