use std::io;

fn get_user_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(value) if value > 0.0 => return value,
            Ok(_) => println!("Please enter a valid positive number."),
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn main() {
    println!("BMI Calculator");

    // Input weight in kilograms
    let weight: f64 = get_user_input("Enter your weight in Kg:");

    // Input height in meters
    let height: f64 = get_user_input("Enter your height in meters:");

    println!("This is the height: {:.2} meters", height);
    println!("This is the weight: {:.2} Kg", weight);
}

