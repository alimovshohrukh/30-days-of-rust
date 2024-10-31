# Day 7: Fibonacci Sequence Generator

## Objective
The goal for Day 7 was to create a program in Rust that generates the Fibonacci sequence up to `n` terms, where each term is calculated using the formula:

\[ F(n) = F(n-1) + F(n-2) \quad \text{for } n > 1 \]

The sequence starts with `0` and `1`, and each subsequent term is the sum of the previous two terms.

## What I Learned
- Using loops to iterate and calculate values based on previous results.
- Managing variables to track consecutive Fibonacci terms.
- Handling user input to ensure the program receives a valid number.

## Program Flow
1. **Prompt for Number of Terms**:
    - The program asks the user to specify how many terms of the Fibonacci sequence they want to generate.
2. **Calculate and Print Fibonacci Terms**:
    - Starting with `0` and `1`, each term is calculated as the sum of the previous two terms.
    - The terms are printed sequentially until reaching the specified count.

## Code Example

```rust
use std::io;

fn main() {
    println!("Enter the number of terms for the Fibonacci sequence:");

    let n = match get_input() {
        Some(num) => num,
        None => {
            println!("Please enter a valid positive number.");
            return;
        }
    };

    if n <= 0 {
        println!("Please enter a positive number greater than zero.");
    } else {
        print_fibonacci(n);
    }
}

fn print_fibonacci(n: usize) {
    let mut f0 = 0;
    let mut f1 = 1;

    print!("Fibonacci sequence: {} ", f0);

    for _ in 1..n {
        print!("{} ", f1);
        let next = f0 + f1;
        f0 = f1;
        f1 = next;
    }
    println!();
}

fn get_input() -> Option<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<usize>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}
