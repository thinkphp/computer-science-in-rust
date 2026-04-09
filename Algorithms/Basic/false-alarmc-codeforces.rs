use std::{cmp::max, io::{self, Write}};

// Reads two space-separated integers from input.
fn read_two_ints() -> (i32, i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.trim().split_whitespace();
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

// Reads a single line of input as a string.
fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let t: i32 = read_line().parse().unwrap(); // Number of test cases

    for _ in 0..t {
        let (n, x) = read_two_ints(); // n = length of the string, x = max allowed length
        let s = read_line().replace(" ", ""); // Remove spaces from the input string

        let mut first = -1;
        let mut last = -1;

        // Find the first and last occurrence of '1' in the string.
        for (i, c) in s.chars().enumerate() {
            if c == '1' {
                first = if first == -1 { i as i32 } else { first };
                last = max(last, i as i32);
            }
        }

        // Check if the distance between the first and last '1' is less than or equal to x.
        if last - first + 1 <= x {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
