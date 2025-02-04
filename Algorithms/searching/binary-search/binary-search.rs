fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let (mut left, mut right) = (0, arr.len().saturating_sub(1)); // Handle empty arrays safely
    while left <= right {
        let mid = left + (right - left) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid.saturating_sub(1),
        }
    }
    None
}

fn main() {
    let arr = [1, 3, 5, 7, 9, 11, 13];
    let target = 7;

    match binary_search(&arr, target) {
        Some(index) => println!("Element found at index {}", index),
        None => println!("Element not found"),
    }
}
