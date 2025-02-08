use std::io;

fn bubble_sort(arr: &mut Vec<i32>) {

     let n = arr.len();

     for i in 0..n {

        let mut swapped = false;

        for j in 0..n-i-1 {

                if arr[j] > arr[j+1] {
                    arr.swap(j, j+1);
                    swapped = true;
                }
        }

        if !swapped {
            break;
        }

     }
}

fn main() {

    println!("Introduceti numarul de elemente: ");

    let mut input = String::new();

    io::stdin()
        .read_line( &mut input )
        .expect("Eroare la citirea numarului de elemente");

    let n: usize = input.trim()
           .parse()
           .expect("numar valid");

    println!("Introduceti {} numere separate prin spatiu: ", n);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Eroare la citirea numerelor");

     let mut numbers: Vec<i32> = input
         .split_whitespace()
         .map(|x| x.parse().expect("Introduceti numere valide"))
         .collect();

     if numbers.len() != n {
         println!("Numarul de elemente introdus nu corespunde cu n");
         return;
     }

     println!("Vectorul initial: {:?}", numbers);

     bubble_sort( &mut numbers );

     println!("Vectorul sortat: {:?}", numbers);
}
