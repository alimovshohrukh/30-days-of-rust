use std::io::stdin as io;
fn main() {
    let num1 = get_input("Enter the first number:");
    let num2 = get_input("Enter the second number:");
    let op = choose_operation();

    let result = calculate(num1, num2, op);
    println!("The result of {} {} {} = {}", num1, op, num2, result);
}
fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);

    let mut input = String::new();
    io().read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse::<f64>()
        .expect("Please enter a valid number")
}
fn choose_operation() -> char {
    println!("Choose an operation (+, -, *, /):");

    let mut op = String::new();
    io().read_line(&mut op)
        .expect("Failed to read line");

    op.trim().chars().next().unwrap()
}
fn add(a: f64, b: f64) -> f64 {
    a + b
}
fn subtract(a: f64, b: f64) -> f64 {
    a - b
}
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}
fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("Cannot divide by zero");
    }
    a / b
}
fn calculate(a: f64, b: f64, op: char) -> f64 {
    match op {
        '+' => add(a, b),
        '-' => subtract(a, b),
        '*' => multiply(a, b),
        '/' => divide(a, b),
        _ => panic!("Invalid operation!"),
    }
}