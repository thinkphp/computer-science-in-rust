use std::env;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use rayon::prelude::*;

/// Function to count occurrences of a word in the given content
fn count_occurrences(content: &str, word: &str) -> usize {
    content
        .split_whitespace()
        .filter(|&w| w == word)
        .count()
}

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure a search word is passed as an argument
    if args.len() < 2 {
        eprintln!("Usage: {} <search_word>", args[0]);
        std::process::exit(1);
    }

    // The search word is passed as the first argument
    let search_word = &args[1];

    // Files to process
    let file_paths = vec![
        "data/file1.txt",
        "data/file2.txt",
        "data/file3.txt",
    ];

    // Shared total count using Mutex and Arc for thread-safe access
    let total_count = Arc::new(Mutex::new(0));

    // Concurrently read file contents using threads
    let mut handles = vec![];
    for file_path in &file_paths {
        let total_count = Arc::clone(&total_count);
        let file_path = file_path.to_string();
        let search_word = search_word.clone();

        let handle = thread::spawn(move || {
            // Read the file content
            let content = fs::read_to_string(&file_path)
                .unwrap_or_else(|_| "".to_string());

            // Count occurrences of the word
            let count = count_occurrences(&content, &search_word);

            // Update the total count safely
            let mut total = total_count.lock().unwrap();
            *total += count;

            println!("Found {} occurrences in {}", count, file_path);
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Display total occurrences after threads complete
    let final_count = *total_count.lock().unwrap();
    println!("Total occurrences of '{}': {}", search_word, final_count);

    // Parallel processing with Rayon
    let parallel_count: usize = file_paths
        .par_iter()
        .map(|file_path| {
            let content = fs::read_to_string(file_path)
                .unwrap_or_else(|_| "".to_string());
            count_occurrences(&content, search_word)
        })
        .sum();

    println!("Parallel total occurrences of '{}': {}", search_word, parallel_count);
}

