fn main() {
    let some_number = Some(5);
    let absent_number: Option<i32> = None;
    let penny = Coin::Penny;
    println!("{}", value_in_cents(penny));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
}

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
}

fn value_in_cents(coin: Coin) -> u8{
    match coin {
        Coin::Penny => {
            println!("lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}