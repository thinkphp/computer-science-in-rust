macro_rules! bubblesort {

       ($arr:expr) => {{

            let mut arr = $arr;

            let len = arr.len();

            for i in 0..len {

                     for j in 0..len-1-i {

                         if arr[j]>arr[j+1] {

                             arr.swap(j, j+1);
                         }
                     }
            }

            arr
       }};
}

fn main() {

    let numbers = vec![34,7,23,32,5,55];

    println!("Before sorting: {:?}", numbers);

    let sorted_numbers = bubblesort!( numbers );

    println!("After sorting: {:?}", sorted_numbers);
}
