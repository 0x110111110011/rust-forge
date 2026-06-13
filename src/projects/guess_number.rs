// Standard I/O library for reading user input
use std::cmp::Ordering;
use std::io;
// Random number generator from the rand crate
use rand::random_range;

pub(crate) fn guess() {
    //     guessing number
    let rnd_number = random_range(1..=100);
    println!("The random number is: {}", rnd_number);

    loop {
        let mut guess_number = String::new();
        println!("Guess a number between 1 and 100:");
        io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to read line");
        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess_number);

        match guess_number.cmp(&rnd_number) {
            Ordering::Less => {
                println!("Too small!");
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => {
                println!("Too big!");
            }
        }
    }
}
