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

    // Check if a number was provided
    if args.len() < 2 {
        eprintln!("Please provide a number to check.");
        return;
    }

    // Parse the first argument as an integer
    let num: i32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Please provide a valid number.");
            return;
        }
    };

    if is_prime!(num) {
        println!("{} is prime!", num);
    } else {
        println!("{} is not prime!", num);
    }
}
