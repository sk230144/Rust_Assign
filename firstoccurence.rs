fn first_occurrence_index(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    
    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] == target {
            while mid > 0 && arr[mid - 1] == target {
                mid -= 1;
            }
            return Some(mid);
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    
    None
}

fn main() {
    let arr = vec![1, 2, 3, 4, 4, 4, 5, 6, 7];
    let target = 4;
    match first_occurrence_index(&arr, target) {
        Some(index) => println!("First occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
