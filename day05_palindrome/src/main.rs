use std::io;

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn clean_string(input: &str) -> String {
    input.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}

fn is_palindrome(input: &str) -> bool {
    let cleaned = clean_string(input);
    cleaned == cleaned.chars().rev().collect::<String>()
}

fn main() {
    let input = get_input("Enter a string to check if it's a palindrome:");
    if is_palindrome(&input) {
        println!("\"{}\" is a palindrome!", input);
    } else {
        println!("\"{}\" is not a palindrome.", input);
    }
}