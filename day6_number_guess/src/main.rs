use rand::Rng;
use std::io::stdin as io;

fn main() {
    let rng = rand::thread_rng().gen_range(1..=100);
    let mut attempts: u32 = 0;
    loop {
        attempts += 1;
        println!("Please enter a number:");
        let number = match get_input() {
            Some(num) => num,
            None => {
                println!("Please enter a valid number.");
                continue;
            }
        };
        if number > rng {
            println!("Too high!");
        } else if number < rng {
            println!("Too low");
        } else {
            println!("Congratulations! You guessed the number! Number of attempts taken: {}", attempts);
            break;
        }
    }
}

fn get_input() -> Option<i32> {
    let mut input = String::new();
    io().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<i32>() {
        Ok(num) => Some(num),
        Err(_) => None,  // Return None if the input is invalid
    }
}
