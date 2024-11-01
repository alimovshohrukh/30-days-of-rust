# Day 8: Vector Operations - Sum, Average, Max, and Min

## Objective
The goal for Day 8 was to create a program in Rust that works with vectors. This program takes a list of numbers as input from the user, stores them in a vector, and then calculates and displays the sum, average, maximum, and minimum values of the numbers.

## What I Learned
- How to create and manipulate vectors in Rust.
- Using iterators to calculate the sum, average, maximum, and minimum values.
- Working with user input, including parsing and handling whitespace-separated numbers.
- Implementing error handling for invalid inputs.

## Program Flow
1. **Prompt for Input**:
    - The program asks the user to enter a list of numbers separated by spaces.
2. **Store Numbers in a Vector**:
    - User input is split and parsed into integers, which are stored in a vector.
3. **Calculate Statistics**:
    - The program calculates the following statistics for the numbers:
        - **Sum**: Total of all numbers.
        - **Average**: Sum divided by the count of numbers.
        - **Maximum**: Largest number in the list.
        - **Minimum**: Smallest number in the list.
4. **Display Results**:
    - Prints the list of numbers, sum, average, maximum, and minimum values.

## Code Example

```rust
use std::io;

fn main() {
    let numbers = get_numbers();
    calculate_stats(&numbers);
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

fn calculate_stats(numbers: &Vec<i32>) {
    if numbers.is_empty() {
        println!("No numbers were entered.");
        return;
    }

    let sum: i32 = numbers.iter().sum();
    let average = sum as f64 / numbers.len() as f64;
    let max = numbers.iter().max().unwrap();
    let min = numbers.iter().min().unwrap();

    println!("Numbers: {:?}", numbers);
    println!("Sum: {}", sum);
    println!("Average: {:.2}", average);
    println!("Maximum: {}", max);
    println!("Minimum: {}", min);
}
