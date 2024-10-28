use std::io::stdin as io;

fn main() {
    let choose_type = choose_conversion("Choose conversion type\nC for Celsius\nF for Fahrenheit");

    let num = get_input("Enter the number:");

    match choose_type {
        'C' => celsius_to_fahrenheit(&num),
        'F' => fahrenheit_to_celsius(&num),
        _ => panic!("Invalid conversion type"),
    }
}

fn fahrenheit_to_celsius(p0: &i16) {
    // C = (F - 32) * 5/9
    let c = (p0 - 32) * 5 / 9;
    println!("{} Fahrenheit in Celsius is: {}", p0, c);
}

fn celsius_to_fahrenheit(p0: &i16) {
    // F = (C * 9/5) + 32
    let f = (p0 * 9 / 5) + 32;
    println!("{} Celsius in Fahrenheit is: {}", p0, f);
}

fn get_input(prompt: &str) -> i16 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io().read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<i16>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn choose_conversion(prompt: &str) -> char {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io().read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<char>() {
            Ok(choice) if choice == 'C' || choice == 'F' => return choice,
            _ => println!("Please enter 'C' for Celsius or 'F' for Fahrenheit."),
        }
    }
}
