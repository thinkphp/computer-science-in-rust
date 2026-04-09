#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::io::Write;
#[allow(unused_imports)]
use std::str;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }

    /// Use "turbofish" syntax next::<T>() to select data type of next token.
    ///
    /// # Panics
    ///
    /// Panics if there's an I/O error or if the token cannot be parsed as T.
    pub fn next<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

#[allow(dead_code)]
pub fn scanner_from_file(filename: &str) -> Scanner<io::BufReader<std::fs::File>> {
    let path = String::from("../") + filename;
    let file = std::fs::File::open(path).expect("Input file not found");
    Scanner::new(io::BufReader::new(file))
}

#[allow(dead_code)]
pub fn writer_to_file(filename: &str) -> io::BufWriter<std::fs::File> {
    let path = String::from("../") + filename;
    let file = std::fs::File::create(path).expect("Output file not found");
    io::BufWriter::new(file)
}

fn solve() {
    let mut reader = scanner_from_file("adunare.in");
    let mut writer = writer_to_file("adunare.out");

    let a: i32 = reader.next();
    let b: i32 = reader.next();
    let c = a + b;

    writeln!(writer, "{}", c).unwrap();
}
