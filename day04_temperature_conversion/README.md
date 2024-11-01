# Day 4: Temperature Converter 🌡️

## Objective
Today's task was to create a program in Rust that converts temperatures between Celsius and Fahrenheit. The program allows the user to choose a conversion type and enter a temperature value, which is then converted and displayed.

## What I Learned
- How to handle user input and validate it using loops.
- Using `match` statements for branching based on user selection.
- Implementing simple mathematical functions for temperature conversion.
- Managing error handling to ensure only valid inputs are accepted.

## Program Flow
1. **Choose Conversion Type**:
    - The user is prompted to choose the type of conversion: Celsius to Fahrenheit (`C`) or Fahrenheit to Celsius (`F`).
2. **Input Temperature**:
    - The program asks the user to enter a temperature value.
3. **Perform Conversion**:
    - Based on the conversion type, the program calculates the converted temperature using a specific formula:
        - **Celsius to Fahrenheit**: `F = (C * 9/5) + 32`
        - **Fahrenheit to Celsius**: `C = (F - 32) * 5/9`
4. **Display Result**:
    - The converted temperature is displayed to the user.

## Code Example

```rust
fn main() {
    let choose_type = choose_conversion("Choose conversion type\nC for Celsius\nF for Fahrenheit");

    let num = get_input("Enter the number:");

    match choose_type {
        'C' => celsius_to_fahrenheit(&num),
        'F' => fahrenheit_to_celsius(&num),
        _ => panic!("Invalid conversion type"),
    }
}