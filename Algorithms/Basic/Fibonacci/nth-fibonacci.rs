//int fibonacci(int n) {}
fn fibonacci_sequence(n: i32) {

    let mut a = 0i32;

    let mut b = 1i32;

    print!("fibonacci sequence up to {}: {} {} ", n, a, b);

    while a + b <= n {

      let temp = a + b;

      print!("{} ", temp);

      a = b;

      b = temp;
     }

      println!();

}

fn fibonacci(n: i32) -> i32 {

    if n == 0i32 {

        return 0i32;

    } else if n == 1i32 {

        return 1i32;
    }

    let mut a = 0i32;

    let mut b = 1i32;

    for _ in 2i32..=n {

        let temp = a + b;

        a = b;

        b = temp;
    }

    b
}
fn main() {

   let n = 10i32;//i de la int

   println!("fibonacci({}) = {}", n, fibonacci(n));

   fibonacci_sequence( n );
}
