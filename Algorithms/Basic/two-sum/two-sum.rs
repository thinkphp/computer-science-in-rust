use std::collections::HashMap;

fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return Some((j, i));
        }
        map.insert(num, i);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(&nums, target), Some((0, 1)));
    }

    #[test]
    fn test_two_sum_no_solution() {
        let nums = vec![1, 2, 3, 4];
        let target = 10;
        assert_eq!(two_sum(&nums, target), None);
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    if let Some((i, j)) = two_sum(&nums, target) {
        println!("Indices: ({}, {})", i, j);
    } else {
        println!("No solution found");
    }
}

