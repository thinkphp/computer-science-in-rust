fn is_palindrome(s: &str) -> bool {
    let s: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let (mut left, mut right) = (0, s.len().saturating_sub(1));
    while left < right {
        if s[left] != s[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

fn main() {
    let test_cases = ["racecar", "hello", "A man, a plan, a canal: Panama", "12321", "not a palindrome"];
    
    for &s in &test_cases {
        println!("\"{}\" is a palindrome? {}", s, is_palindrome(s));
    }
}
