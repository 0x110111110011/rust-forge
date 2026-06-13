use std::io;

pub(crate) fn fibonacci_number() {
    loop {
        println!("Welcome to Fibonacci calculation system");
        let mut user_input = String::new();

        println!("Enter your nth number: ");
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input: u8 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if user_input == 0 {
            println!("Calculation of 0: {user_input}");
            break;
        } else if user_input == 1 {
            println!("Calculation of 1: {user_input}");
            break;
        }
        let mut previous_number = 0_u8;
        let mut current_number = 1_u8;

        for _ in 2..=user_input {
            let next_number = previous_number + current_number;
            previous_number = current_number;
            current_number = next_number;
        }
        println!("Fibonacci calculation of {user_input}th is {current_number}");
        break;
    }
}
