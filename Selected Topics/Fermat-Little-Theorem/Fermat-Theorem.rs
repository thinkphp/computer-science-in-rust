use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

// -------------------- SIMPLE RNG (LCG) --------------------
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Lcg { state: seed }
    }

    fn next(&mut self, low: i64, high: i64) -> i64 {
        // Parametri clasici LCG (aceiasi ca glibc)
        self.state = self.state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let r = (self.state >> 33) as i64;
        low + r % (high - low + 1)
    }
}

// -------------------- CHECK (brute force) --------------------
fn check(n: i64) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n < 2 || n % 2 == 0 {
        return false;
    }
    let mut i = 3i64;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

// -------------------- FAST POWER (modular exponentiation) --------------------
fn fast_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    let mut res = 1i64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            res = res * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    res
}

// -------------------- FERMAT TEST --------------------
fn fermat(n: i64, rng: &mut Lcg) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    for _ in 0..5 {
        let a = rng.next(2, n - 2); // [2, n-2]
        if fast_pow(a, n - 1, n) != 1 {
            return false;
        }
    }
    true
}

// -------------------- MAIN --------------------
fn main() {
    // Seed din timpul sistemului (echivalent srand(time(0)))
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let mut rng = Lcg::new(seed);

    print!("N = ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i64 = input.trim().parse().expect("Introduceti un numar valid!");

    println!("Number: {}", n);

    print!("Brute Force: ");
    if check(n) {
        println!("is Prime");
    } else {
        println!("is Not Prime");
    }

    print!("Fermat Test: ");
    if fermat(n, &mut rng) {
        println!("Probably Prime");
    } else {
        println!("Not Prime");
    }
}
