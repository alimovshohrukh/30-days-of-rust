use std::io;

fn main() {
    println!("Enter the number of terms for the Fibonacci sequence:");

    let n = match get_input() {
        Some(num) => num,
        None => {
            println!("Please enter a valid positive number.");
            return;
        }
    };

    if n <= 0 {
        println!("Please enter a positive number greater than zero.");
    } else {
        print_fibonacci(n);
    }
}

fn print_fibonacci(n: usize) {
    let mut f0 = 0;
    let mut f1 = 1;

    print!("Fibonacci sequence: {} ", f0);

    for _ in 1..n {
        print!("{} ", f1);
        let next = f0 + f1;
        f0 = f1;
        f1 = next;
    }
    println!();
}

fn get_input() -> Option<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<usize>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}
