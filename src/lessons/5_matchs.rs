use std::io::ErrorKind::Other;

#[derive(Debug)]
enum UsState {
    Albama,
    Alaska,
    NewYork,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
// fn reroll() {}
// fn move_player(num_spaces: u8) {}

fn dices_roll() {
    let dice_roll = 3;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // _ => reroll(), // other => move_player(other),
    };
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

pub(crate) fn matches() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState::Albama));

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);
    plus_one(Some(5));
    plus_one(None);
}
