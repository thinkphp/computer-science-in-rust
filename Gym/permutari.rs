use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn bkt(pos: usize, n: usize, sol: &mut Vec<usize>, visited: &mut Vec<bool>, writer: &mut BufWriter<File>) {
    if pos > n {
        for i in 1..=n {
            write!(writer, "{} ", sol[i]).unwrap();
        }
        writeln!(writer).unwrap();
        return;
    }

    for val in 1..=n {
        if visited[val] {
            continue;
        }

        sol[pos] = val;
        visited[val] = true;

        bkt(pos + 1, n, sol, visited, writer);

        visited[val] = false;
    }
}

fn main() {
    let fin = File::open("permutari.in").unwrap();
    let mut fout = BufWriter::new(File::create("permutari.out").unwrap());

    let mut reader = BufReader::new(fin);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();

    let mut sol = vec![0; n + 1];
    let mut visited = vec![false; n + 1];

    bkt(1, n, &mut sol, &mut visited, &mut fout);
}
