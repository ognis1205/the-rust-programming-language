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
        Coin::Quarter,
        value_in_cents(Coin::Quarter)
    );
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
