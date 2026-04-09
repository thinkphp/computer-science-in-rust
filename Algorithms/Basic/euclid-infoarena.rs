use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn euclid(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn main() {
    let fin = File::open("euclid2.in").expect("nu pot deschide fisierul de input");
    let fout = File::create("euclid2.out").expect("nu pot crea fisierul de output");

    let mut reader = BufReader::new(fin);
    let mut writer = BufWriter::new(fout);

    let mut input = String::new();

    // citesc T
    reader.read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input).unwrap();

        let mut nums = input.split_whitespace();
        let mut a: i32 = nums.next().unwrap().parse().unwrap();
        let mut b: i32 = nums.next().unwrap().parse().unwrap();

        let result = euclid(a, b);
        writeln!(writer, "{}", result).unwrap();
    }
}
