fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    merge(arr, mid);
}

fn merge(arr: &mut [i32], mid: usize) {
    let (left, right) = (arr[..mid].to_vec(), arr[mid..].to_vec());
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    arr[k..].copy_from_slice(if i < left.len() { &left[i..] } else { &right[j..] });
}
