use std::io;

pub(crate) fn convert() {
    // 1. input 1, 2, 3 -> 1 = f2c, 2 = c2f, 3 = quit
    #[derive(Debug)]
    enum Choice {
        F2C,
        C2F,
        Quit,
    }
    const CHOICES: [u8; 3] = [1, 2, 3];

    println!("Welcome to Temperature Converter");
    println!("--------------------------------");

    loop {
        println!("Please select an option.");
        println!("1. Fahdrenheit(°F) to Celcius(°C)");
        println!("2. Celcius(°C) to Fahdrenheit(°F)");
        println!("3. Quit");

        let mut user_choice = String::new();

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Please select only specific option.");
        let user_choice: u8 = match user_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You selected: {user_choice}");

        if user_choice == 1 {
            println!("Enter Fahdrenheit(°F): ");

            let mut user_choice = String::new();

            io::stdin()
                .read_line(&mut user_choice)
                .expect("Enter a valid Fahdrenheit.");
            let user_choice: f32 = match user_choice.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("You entered Fahdrenheit: {user_choice}°F");

            let celcius = f2c_converter(user_choice);
            println!("Converted to Celcius: {celcius}°C");
        } else if user_choice == 2 {
            println!("Enter Celcius(°C): ");

            let mut user_choice = String::new();

            io::stdin()
                .read_line(&mut user_choice)
                .expect("Enter a valid Celcius.");
            let user_choice: f32 = match user_choice.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("You entered Celcius: {user_choice}°C");

            let fahrenheit = c2f_converter(user_choice);
            println!("Converted to Fahdrenheit: {fahrenheit}°F");
        } else if user_choice == 3 {
            println!("Quitting...");
            break;
        } else {
            println!("Invalid input!!!");
        }
    }
}

fn f2c_converter(fahrenheit: f32) -> f32 {
    ((fahrenheit - 32.0) * 5.0) / 9.0
}
fn c2f_converter(celcius: f32) -> f32 {
    ((celcius * 9.0) / 5.0) + 32.0
}
