#[derive(Debug)]
enum UsState {
    Albama,
    Alaska,
    NewYork,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Albama => year >= 1819,
            UsState::Alaska => year >= 1959,
            UsState::NewYork => year >= 1989,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_state_quarter_if_let(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
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

    describe_state_quarter(Coin::Quarter(UsState::NewYork));
    describe_state_quarter_if_let(Coin::Quarter(UsState::NewYork));
}
