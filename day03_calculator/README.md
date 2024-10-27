# Day 3: Basic Calculator

## Objective
The goal for Day 3 was to create a simple calculator in Rust that can handle basic operations: addition, subtraction, multiplication, and division. The program takes two numbers and an operator as input, performs the selected operation, and outputs the result.

## What I Learned
- How to capture user input and convert it into numbers for calculations.
- Using `match` statements to execute different functions based on user input.
- Writing separate functions for each operation to keep the code modular.
- Error handling to manage division by zero and invalid operations.

## Steps to Build
1. **Take two numbers as inputs**: Used `std::io` to read user input and convert strings to `f64` numbers.
2. **Choose an operation**: Allowed the user to select an operation (`+`, `-`, `*`, `/`) with simple input.
3. **Define functions**: Created individual functions for addition, subtraction, multiplication, and division.
4. **Perform calculation**: Used a `match` statement to call the appropriate function based on the chosen operation.
5. **Handle errors**: Added error handling for invalid operations and division by zero.

## Code Example
Below is a brief example of the main logic for performing calculations based on user input:

```rust
fn main() {
    let num1 = get_input("Enter the first number:");
    let num2 = get_input("Enter the second number:");
    let op = choose_operation();

    let result = calculate(num1, num2, op);
    println!("The result of {} {} {} = {}", num1, op, num2, result);
}
