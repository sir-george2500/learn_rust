use std::io;
use colored::Colorize;
use rand::Rng;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The Secret Number is {}", secret_number);
    println!("Please enter your guess");

    loop {

        let mut guess = String::new();


        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                return;
            }
        };


        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("{}","Too small!".red()),
            std::cmp::Ordering::Greater => println!("{}","Too large!".red()),
            std::cmp::Ordering::Equal => {
                println!("{}","You got it!".green());
                break;
            }
        }
    }
}

