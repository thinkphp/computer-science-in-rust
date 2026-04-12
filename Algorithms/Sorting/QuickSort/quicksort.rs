use std::fs::File;
use std::io::{self, Read, Write};

fn quicksort(vec: &mut [i32], lo: isize, hi: isize) {
    let mut i = lo;
    let mut j = hi;

    let pivot = vec[((lo + hi) / 2) as usize];

    while i <= j {
        while vec[i as usize] < pivot {
            i += 1;
        }
        while vec[j as usize] > pivot {
            j -= 1;
        }

        if i <= j {
            vec.swap(i as usize, j as usize);
            i += 1;
            j -= 1;
        }
    }

    if lo < j {
        quicksort(vec, lo, j);
    }
    if i < hi {
        quicksort(vec, i, hi);
    }
}

fn main() -> io::Result<()> {
    // citire din fișier
    let mut input = String::new();
    File::open("algsort.in")?.read_to_string(&mut input)?;

    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut vec: Vec<i32> = Vec::with_capacity(n);

    for _ in 0..n {
        vec.push(it.next().unwrap().parse().unwrap());
    }

    if n > 0 {
        quicksort(&mut vec, 0, (n as isize) - 1);
    }

    // scriere în fișier
    let mut output = File::create("algsort.out")?;
    for x in vec {
        write!(output, "{} ", x)?;
    }

    Ok(())
}
