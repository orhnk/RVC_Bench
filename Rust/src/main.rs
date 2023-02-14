/* File: main.rs 
 * Purpuse: This file serves its idenity as the main rust program.
 * Author KoBruhh
 * Date: 06.02.2023
 * */



/* 1.70, 1.71, 1.70 -> Rust (Unsafe! handled in the same way as C) --release mode
 * 1.73, 1.80, 1.73 -> Rust (Fully safe) --release mode
 * 1.98, 2.00, 2.06 -> V (Fully safe) -prod mode
 * 2.20, 2.21, 2.18 -> C (Unsafe! handled in the same way as Rust) --Ofast mode
 * 4.26, 4.44, 4.25 -> C (Unsafe! handled in the same way as Rust) --O0 mode (Unoptimized)
 * 11.70, 11.69, 11.77 -> V (Fully safe) --standard mode (Unoptimized)
 * 20.54, 20.16, 20.08 -> Rust (Unsafe handled in the same wat as C) --debug mode (Unoptimized)
 * 23.56, 23.09, 23.19 -> Rust (Fully safe) --debug mode (Unoptimized)
 * */

/* Version Information:
 * Rustup: 1.66.0 (12/12/2022)
 * V: 0.3.3 -> Yes. V did not hit the stable yet.
 * GCC: 12.2.1
 * */

const SIZE: usize = 100000; // -> same as in C
fn main() {
    let mut numbers = Vec::with_capacity(SIZE);
    for i in 0..SIZE {
        numbers.push(100000 - i as i32);
    }
    quick_sort(&mut numbers);
}
fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = 100000;
    _quick_sort(arr, 0, (len - 1) as isize);
}
fn _quick_sort<T: Ord>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = partition(arr, low, high);
        _quick_sort(arr, low, p - 1);
        _quick_sort(arr, p + 1, high);
    }
}
fn partition<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;
    loop {
        store_index += 1;
        while arr[store_index as usize] < arr[pivot] {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && arr[last_index as usize] > arr[pivot] {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize);
        }
    }
    arr.swap(store_index as usize, pivot as usize);
    store_index
}
