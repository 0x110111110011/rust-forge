// mod guess_number;

// #[path = "lessons/1_variables.rs"]
// mod variables;

// #[path = "lessons/2_loops.rs"]
// mod loops;

// #[path = "projects/temperature_converter.rs"]
// mod temperature_converter;

// #[path = "projects/fibonacci_number.rs"]
// mod fibonacci_number;

#[path = "projects/the_12_days_of_christmas.rs"]
mod the_12_days_of_christmas;

fn main() {
    // 1. Guessing game
    // guess_number::guess();

    // 2. Variable
    // variables::variable();

    // 3. Loop
    // loops::loops();

    // 4. Temperature Converter
    // temperature_converter::convert();

    // 5. Fibonacci Number
    // fibonacci_number::fibonacci_number();

    // 6. The 12 days of Christmas
    the_12_days_of_christmas::christmas_present();
}
