use std::env;

macro_rules! is_prime {
    ($n:expr) => {{
        let mut is_prime = true;
        if $n <= 1 {
            is_prime = false;
        } else {
            for i in 2..=($n as f64).sqrt() as i32 {
                if $n % i == 0 {
                    is_prime = false;
                    break;
                }
            }
        }
        is_prime
    }};
}

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a number (n) was provided
    if args.len() < 2 {
        eprintln!("Please provide a number of primes to display.");
        return;
    }

    // Parse the first argument as an integer (the number of primes to display)
    let n: i32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Please provide a valid number.");
            return;
        }
    };

    if n <= 0 {
        eprintln!("Please provide a positive number.");
        return;
    }

    // Find and display the first n primes
    let mut count = 0;
    let mut num = 2; // Starting from the first prime number

    while count < n {
        if is_prime!(num) {
            println!("{}", num);
            count += 1;
        }
        num += 1;
    }
}
