# Day 2: Recursion - Factorial Calculation

## Objective
Implement a function in Rust to calculate the factorial of a given number using recursion.

## What I Learned
- Practiced writing recursive functions in Rust.
- Learned about base cases and how recursion can build solutions step-by-step.

## Code
Here’s the Rust code to calculate the factorial of a number:

```rust
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let number = 5; 
    println!("The factorial of {} is {}", number, factorial(number));
}
