enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        other => {
            println!("no match");
            None
        }
    }
}

fn plus_one_scope() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn control_flow_if_let() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max)
    }

    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter => println!("State quarter"),
        _ => count += 1,
    }

    if let Coin::Quarter = coin {
        println!("State quarter");
    } else {
        count += 1;
    }
}
