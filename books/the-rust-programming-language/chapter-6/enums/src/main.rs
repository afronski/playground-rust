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
    }
}

fn main() {
    println!("Value in cents: {}", value_in_cents(Coin::Penny));
    println!("Value in cents: {}", value_in_cents(Coin::Nickel));
    println!("Value in cents: {}", value_in_cents(Coin::Dime));
    println!("Value in cents: {}", value_in_cents(Coin::Quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(5) = five {
        println!("It's five!");
    }

    if let Some(6) = six {
        println!("It's six!");
    }

    if let None = none {
        println!("It's none!");
    }

    let some_u8_value = 0u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
