fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();

    if len <= 1 {
        return;
    }

    // Choose the last element as the pivot.
    let pivot = arr[len - 1];
    let mut i = 0;

    // Move smaller elements to the left.
    for j in 0..len - 1 {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    // Place the pivot in its final sorted position.
    arr.swap(i, len - 1);

    // Sort both sides, excluding the pivot.
    quick_sort(&mut arr[..i]);
    quick_sort(&mut arr[i + 1..]);
}

fn main() {
    let mut arr = [17, 12, 6, 19, 23, 8, 5, 10];

    println!("Before sorting: {:?}", arr);

    quick_sort(&mut arr);

    println!("After sorting:  {:?}", arr);
}
