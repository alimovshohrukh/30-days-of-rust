# Day 9: Finding Largest and Smallest Numbers in an Array

## Objective
The goal for Day 9 was to create a program in Rust that finds the largest and smallest numbers from a user-inputted list of numbers. The program takes a series of numbers separated by spaces, stores them in a vector, and identifies the maximum and minimum values.

## What I Learned
- How to parse and handle a list of numbers input by the user.
- Using Rust’s iterator methods (`iter().max()` and `iter().min()`) to find the largest and smallest values.
- Implementing basic error handling to ensure the program behaves correctly with an empty list.

## Program Flow
1. **Prompt for Input**:
    - The program asks the user to enter a list of numbers separated by spaces.
2. **Store Numbers in a Vector**:
    - The input string is split by whitespace, and each item is parsed and stored as an integer in a vector.
3. **Find Largest and Smallest Numbers**:
    - The program calculates the maximum and minimum values in the vector using Rust’s iterator methods.
4. **Display Results**:
    - Prints the list of numbers along with the largest and smallest values.

## Code Example

```rust
use std::io;

fn main() {
    let numbers = get_numbers();
    find_min_max(&numbers);
}

fn get_numbers() -> Vec<i32> {
    println!("Enter a list of numbers separated by spaces:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

fn find_min_max(numbers: &Vec<i32>) {
    if numbers.is_empty() {
        println!("No numbers were entered.");
        return;
    }

    let max = numbers.iter().max().unwrap();
    let min = numbers.iter().min().unwrap();

    println!("Numbers: {:?}", numbers);
    println!("Largest number: {}", max);
    println!("Smallest number: {}", min);
}
