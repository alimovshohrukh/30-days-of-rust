# Day 5: Palindrome Checker

## Objective
The goal for Day 5 was to create a program in Rust that checks if a given string is a palindrome. A palindrome is a string that reads the same backward as forward, ignoring spaces, punctuation, and case.

## What I Learned
- How to manipulate and clean strings in Rust.
- Using Rust's `chars()` iterator to filter characters and reverse strings.
- Writing a simple logic to compare strings for palindrome properties.

## Program Flow
1. **Prompt for Input**: Ask the user to enter a string.
2. **Clean the String**:
    - Convert to lowercase and remove non-alphanumeric characters to standardize the input.
3. **Check Palindrome**:
    - Reverse the cleaned string and compare it with the original cleaned string.
4. **Display Result**:
    - Print whether the input string is a palindrome or not.

## Code Example

```rust
fn main() {
    let input = get_input("Enter a string to check if it's a palindrome:");
    if is_palindrome(&input) {
        println!("\"{}\" is a palindrome!", input);
    } else {
        println!("\"{}\" is not a palindrome.", input);
    }
}
