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
