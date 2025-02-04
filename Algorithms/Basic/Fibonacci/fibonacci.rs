fn fibonacci(n: u32) -> u64 {
    if n == 0 { return 0; }
    let (mut a, mut b) = (0, 1);
    for _ in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}
