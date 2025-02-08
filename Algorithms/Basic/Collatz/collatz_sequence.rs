use std::io;

fn collatz_sequence(mut n: i64) {

    print!("Collatz sequence starting from {}: {}", n, n);

    while n != 1 {

        if n % 2 == 0 {
            n = n / 2;

        } else {
            n = 3 * n + 1;
        }
        print!(" {} ", n);
    }

    println!();

}

fn main() {

    println!("Enter a positive number:");

    let mut input = String::new();//empty string

    io::stdin().read_line(&mut input).expect("Failed to read line");

    //convertim input la i64 adica la intreg pe 64 de biti
    let n: i64 = input.trim().parse().expect("Please enter a valid number");

    if n <= 0 {
        println!("Please enter a positive number");
        return;
    }

    collatz_sequence(n);
}
