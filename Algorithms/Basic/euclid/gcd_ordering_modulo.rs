use std::cmp::Ordering;

fn gcd(a: u32, b: u32) -> u32 {
    match b.cmp(&0) {
        Ordering::Equal => a, // Dacă b este 0, GCD este a
        _ => gcd(b, a % b), // Altfel, continuăm cu b și a % b
    }
}

fn main() {
    let a = 56;
    let b = 98;
    println!("GCD of {} and {} is {}", a, b, gcd(a, b));
}
