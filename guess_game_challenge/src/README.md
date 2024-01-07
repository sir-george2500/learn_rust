# Number Guessing Game

A command-line based number guessing game written in Rust.

## How to Play

1. Clone the repository or copy the code into a Rust file.
2. Ensure you have Rust installed on your system.
3. Run the program using `cargo run` or by compiling and executing the binary.

## Game Rules

1. The program will generate a random secret number between 1 and 100.
2. The player is prompted to input their guess.
3. After each guess, the program provides feedback on whether the guess was too small, too large, or correct.
4. The game continues until the player correctly guesses the secret number.
5. The player is informed of their success, and the game ends.

## Dependencies

- `rand`: Used to generate random numbers.
- `colored`: Used to add color to the console output for a better user experience.

## Installation

If you don't have the required dependencies, you can add them to your `Cargo.toml` file:

```toml
[dependencies]
rand = "0.8"
colored = "2.0"

