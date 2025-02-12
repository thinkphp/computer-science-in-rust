use std::io::{self, Write};
use rand::Rng;
use std::collections::HashSet;

fn play_game() -> bool {
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..=100);
    let mut attempts = 0;
    const MAX_ATTEMPTS: u32 = 10;
    let mut previous_guesses = HashSet::new();

    println!("\nWelcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 100. You have {MAX_ATTEMPTS} attempts.");
    println!("Can you guess it?\n");

    while attempts < MAX_ATTEMPTS {
        print!("Attempt {}/{MAX_ATTEMPTS}. Enter your guess: ", attempts + 1);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Parse and validate input
        let guess: i32 = match input.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    println!("Please enter a number between 1 and 100.");
                    continue;
                }
                if !previous_guesses.insert(num) {
                    println!("You already tried that number! Try something else.");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        // Track the attempt
        attempts += 1;

        // Check the guess
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Equal => {
                println!("\nCongratulations! You've guessed the number in {attempts} attempts!");
                return true;
            }
            std::cmp::Ordering::Less => println!("Too low!"),
            std::cmp::Ordering::Greater => println!("Too high!"),
        }

        // Show remaining attempts
        if attempts < MAX_ATTEMPTS {
            println!("You have {} attempts remaining.", MAX_ATTEMPTS - attempts);
        }

        // Show previous guesses
        let mut sorted_guesses: Vec<i32> = previous_guesses.iter().cloned().collect();
        sorted_guesses.sort();
        println!("Your previous guesses: {:?}\n", sorted_guesses);
    }

    println!("\nGame Over! The number was {secret_number}.");
    false
}

fn main() {
    let mut score = 0;

    loop {
        if play_game() {
            score += 1;
        }

        print!("\nWould you like to play again? (yes/no): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().to_lowercase() != "yes" {
            println!("\nThanks for playing! You won {score} game(s)!");
            break;
        }
    }
}
