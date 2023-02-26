use std::mem::MaybeUninit;

const SIZE: usize = 100000; // -> same as in C
fn main() {
    let mut numbers: [i32; 100000] = unsafe{ MaybeUninit::uninit().assume_init() }; // -> initializing memory same as in C
    for i in 0..SIZE {
        numbers[i] = 100000 - i as i32;
    }
    unsafe {
        std::mem::transmute::<_, [i32; 100000]>(numbers);
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
        unsafe {
        while arr.get_unchecked(store_index as usize) < arr.get_unchecked(pivot) {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && arr.get_unchecked(last_index as usize) > arr.get_unchecked(pivot) {
            last_index -= 1;
        }
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

