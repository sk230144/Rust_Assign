fn reverse_string(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut right = chars.len() - 1;

    while left < right {
        chars.swap(left, right);
        left += 1;
        right -= 1;
    }

    chars.iter().collect()
}

fn main() {
    let s = String::from("hello");
    let reversed = reverse_string(s);
    println!("Reversed string: {}", reversed);
}
