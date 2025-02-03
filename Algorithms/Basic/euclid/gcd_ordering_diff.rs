use std::cmp::Ordering;

fn gcd(a: u32, b: u32) -> u32 {
    match a.cmp(&b) {
        Ordering::Equal => a, // Dacă numerele sunt egale, GCD este oricare dintre ele
        Ordering::Greater => gcd(a - b, b), // Dacă a > b, scădem b din a
        Ordering::Less => gcd(a, b - a), // Dacă a < b, scădem a din b
    }
}

fn main() {
    let a = 56;
    let b = 98;
    println!("GCD of {} and {} is {}", a, b, gcd(a, b));
}
