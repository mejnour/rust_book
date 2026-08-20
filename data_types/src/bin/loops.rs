fn for_loop_countdown() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("elemn: {element}");
    }
}

fn collection_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("{}", a[index]);
        index += 1;
    }
}

fn cond_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!");
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
}

fn return_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter  == 10 {
            break counter * 2;
        }
    };

    println!("result: {result}");
}

fn main() {
    let mut a = 1;
    loop {
        println!("again!");
        if a > 2 {
            break;
        }
        a = a + 1;
        continue;
    }
    println!("a: {a}");

    return_value();
    println!("- - - - - - - - -");
    loop_labels();
    println!("- - - - - - - - -");
    cond_loop();
    println!("- - - - - - - - -");
    collection_loop();
    println!("- - - - - - - - -");
    for_loop();
    println!("- - - - - - - - -");
    for_loop_countdown();
}