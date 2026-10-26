fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();

    if len <= 1 {
        return;
    }

    let mid = len / 2;

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut merged = Vec::with_capacity(len);
    let mut i = 0;
    let mut j = mid;

    while i < mid && j < len {
        if arr[i] <= arr[j] {
            merged.push(arr[i]);
            i += 1;
        } else {
            merged.push(arr[j]);
            j += 1;
        }
    }

    merged.extend_from_slice(&arr[i..mid]);
    merged.extend_from_slice(&arr[j..]);

    arr.copy_from_slice(&merged);
}

fn main() {
    let mut arr = [8, 3, 5, 1];

    println!("Before: {:?}", arr);
    merge_sort(&mut arr);
    println!("After:  {:?}", arr);
}
