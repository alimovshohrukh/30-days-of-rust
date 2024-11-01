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
