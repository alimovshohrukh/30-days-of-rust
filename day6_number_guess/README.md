# Day 6: Guess the Number Game 🎲

## Objective
The goal for Day 6 was to create a "Guess the Number" game in Rust. In this game, the program generates a random number between 1 and 100, and the player attempts to guess it. After each guess, the player receives feedback ("Too high!" or "Too low!") until they guess the correct number.

## What I Learned
- Using the `rand` crate to generate random numbers.
- Implementing a loop that continues until a condition is met.
- Managing user input and handling parsing errors in Rust.
- Tracking and displaying the number of attempts taken to guess the number.

## Program Flow
1. **Generate Random Number**:
    - A random number is generated between 1 and 100, which the player needs to guess.
2. **Prompt for Guesses**:
    - The player enters a guess, which is read and parsed into an integer.
3. **Provide Feedback**:
    - If the guess is higher than the target, the program prints "Too high!".
    - If the guess is lower, it prints "Too low!".
    - If the guess matches, it congratulates the player and shows the number of attempts taken.
4. **End Game**:
    - Once the correct number is guessed, the loop breaks, ending the game.

## Code Example

```rust
use rand::Rng;
use std::io::stdin as io;

fn main() {
    let rng = rand::thread_rng().gen_range(1..=100);
    let mut _attempts: u32 = 0;
    loop {
        _attempts += 1;
        println!("Please enter a number:");
        let number = get_input();
        if number > rng {
            println!("Too high!");
        } else if number < rng {
            println!("Too low");
        } else {
            println!("Congratulations! You guessed the number! Number of attempts taken: {}", _attempts);
            break;
        }
    }
}

fn get_input() -> i32 {
    let mut input = String::new();
    io().read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse::<i32>()
        .expect("Please enter a valid number")
}
