fn bubble_sort<T: Ord>(values: &mut[T]) {

    let mut n = values.len() - 1;

    let mut swapped;

    let mut finished = false;
 
    while !finished {

        swapped = false;
 
        for i in 0..n {

            if values[ i ] > values[i+1] {

                values.swap(i, i + 1);

                swapped = true;
            }
        }
 
        if swapped {

            n = n - 1;

        } else {

           finished = true;
        }
    }
}
 
fn main() {

    let variable = 2;
    println!("{}", variable);

    let mut numbers = [8, 7, 1, 2, 9, 3, 4, 5, 0, 6];
    println!("Before: {:?}", numbers);
 
    bubble_sort(&mut numbers);
    println!("After: {:?}", numbers);

    // Sort strings.
    let mut strings = ["empty", "beach", "art", "car", "deal"];
    println!("Before: {:?}", strings);

    bubble_sort(&mut strings);
    println!("After: {:?}", strings);

}
