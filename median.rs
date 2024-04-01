fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid = len / 2;
        let median = (arr[mid - 1] + arr[mid]) as f64 / 2.0;
        median
    } else {
        let mid = len / 2;
        arr[mid] as f64
    }
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5, 6];
    
    println!("Median of arr1: {}", find_median(&arr1)); // Output: 3.0
    println!("Median of arr2: {}", find_median(&arr2)); // Output: 3.5
}
