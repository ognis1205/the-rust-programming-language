fn main() {
    println!(
        "result: {:?} => {}",
        Coin::Penny,
        value_in_cents(Coin::Penny)
    );
    println!(
        "result: {:?} => {}",
        Coin::Nickel,
        value_in_cents(Coin::Nickel)
    );
    println!("result: {:?} => {}", Coin::Dime, value_in_cents(Coin::Dime));
    println!(
        "result: {:?} => {}",
        Coin::Quarter(State::Alabama),
        value_in_cents(Coin::Quarter(State::Alabama))
    );

    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        println!("coin is not quarter");
    }
}

#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state from: {:?}", state);
            25
        }
    }
}
