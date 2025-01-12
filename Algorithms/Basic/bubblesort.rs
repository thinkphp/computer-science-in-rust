macro_rules! bubble_sort {

    ($arr:expr) => {{
    
        let mut arr = $arr;
        
        let len = arr.len();
        
        for i in 0..len {
        
            for j in 0..len - 1 - i {
            
                if arr[j] > arr[j + 1] {
                
                    arr.swap(j, j + 1);
                }
            }
        }
        arr
    }};
}

fn main() {
    let numbers = vec![34, 7, 23, 32, 5, 62];

    println!("Before sorting: {:?}", numbers);

    // Use the bubble_sort macro to sort the array
    let sorted_numbers = bubble_sort!(numbers);

    println!("After sorting: {:?}", sorted_numbers);
}
