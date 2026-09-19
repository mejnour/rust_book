#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

fn describe_state_quarter_2(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        None
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn describe_state_quarter_3(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => {
            println!("The maximum is configured to be {max}")
        }
        _ => (),
    }

    // o if..let abaixo se comporta igual ao match acima
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}")
    }

    let c1 = Coin::Quarter(UsState::Alaska);
    let mut _count = 0;
    match c1 {
        Coin::Quarter(ref state1) => println!("State quarter from {state1:?}!"),
        _ => _count += 1,
    }

    // o construto abaixo é igual ao de cima
    let mut _count = 0;
    if let Coin::Quarter(ref state2) = c1 {
        println!("State quarter from {state2:?}!")
    } else {
        _count += 1;
    }

    println!("{:?}", describe_state_quarter(c1));
}