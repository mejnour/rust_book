#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    Teste
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
        Coin::Teste => {
            println!("Teste");
            0
        } // quando se usa chaves dentro de um match,
    //     a vírgula depois de um braço é opcional
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // matchs precisam cobrir todos os cenarios
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
fn dices() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other), // qqr outro valor faz bind em other e passa como param
        _ => reroll(), // qqr outro valor que nao será utilizado
        // _ => (), // quando nao queremos que nada aconteca
    }
}

fn main() {
    let c1 = Coin::Nickel;
    println!("c1: {}", value_in_cents(c1));

    let c2 = Coin::Teste;
    println!("c2: {}", value_in_cents(c2));

    let c3 = Coin::Quarter(UsState::Alaska);
    println!("c3: {}", value_in_cents(c3));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?}", five);
    println!("six: {:?}", six);
    println!("none: {:?}", none);
}