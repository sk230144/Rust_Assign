fn is_palindrome(s: &str) -> bool {
    let filtered_chars: String = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    
    filtered_chars == filtered_chars.chars().rev().collect::<String>()
}

fn main() {
    println!("{}", is_palindrome("A man, a plan, a canal: Panama"));
    println!("{}", is_palindrome("racecar"));
    println!("{}", is_palindrome("hello"));
}
