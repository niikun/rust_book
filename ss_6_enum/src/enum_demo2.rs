#![allow(dead_code)]
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(us_state) => {
            println!("State quarter from {:?}",us_state);
            25
        }
    }
}

pub fn run(){
    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value of coin is {}",value_in_cents(coin));
}