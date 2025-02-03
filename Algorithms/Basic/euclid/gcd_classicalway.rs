fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let a = 56;
    let b = 98;
    println!("GCD of {} and {} is {}", a, b, gcd(a, b));
}
