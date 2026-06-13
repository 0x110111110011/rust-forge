// mod guess_number;

// #[path = "lessons/1_variables.rs"]
// mod variables;

// #[path = "lessons/2_loops.rs"]
// mod loops;

#[path = "projects/temperature_converter.rs"]
mod temperature_converter;

fn main() {
    // 1. Guessing game
    // guess_number::guess();
    //
    // 2. Variable
    // variables::variable();
    //
    // 3. Loop
    // loops::loops();

    temperature_converter::convert();
}
