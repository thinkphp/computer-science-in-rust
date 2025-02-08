use std::io;

fn fibonacci_sequence(n: i32) {

    let mut a = 0i32;

    let mut b = 1i32;

    print!("Fibonacci sequence up to {}: {} {}",n, a, b);

    while a + b <= n {

        let temp = a + b;

        print!(" {}", temp);

        a = b;

        b = temp;
    }

    println!();
}

fn main() {

   println!("Enter a number:");

   let mut input = String::new();

   io::stdin().read_line(&mut input).expect("Failed to read line");

   let n: i32 = input.trim().parse().expect("Please enter a valid number");

   fibonacci_sequence(n);
}
