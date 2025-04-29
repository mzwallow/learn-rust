enum Coin_1 {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents_1(coin: Coin_1) -> u8 {
    match coin {
        Coin_1::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin_1::Nickel => 5,
        Coin_1::Dime => 10,
        Coin_1::Quarter => 25,
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin_2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_2(coin: Coin_2) -> u8 {
    match coin {
        Coin_2::Penny => 1,
        Coin_2::Nickel => 5,
        Coin_2::Dime => 10,
        Coin_2::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    value_in_cents_1(Coin_1::Penny);

    // Patterns That Bind to Values
    value_in_cents_2(Coin_2::Quarter(UsState::Alaska));

    // Matching with `Option<T>`
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {five:?}, six: {six:?}, none: {none:?}");

    // Catch-all Patterns and the _ Placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // or `_ => reroll(),`
        // or `_ => (),`
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}
