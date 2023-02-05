use std::{
    mem::{transmute, MaybeUninit},
    ptr::write, ops::Index,
}; // -> initializing memory same as in C

const SIZE: i32 = 100000; // -> same as in C
fn main() {
    println!("Sort numbers ascending");
    let mut numbers = MaybeUninit::<[i32; 100000]>::uninit();
    unsafe {
        initialize_array(&mut numbers);
    }
    println!("Before: {:?}", numbers);
    //quick_sort(&mut numbers);
    println!("After:  {:?}\n", numbers);
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
unsafe fn initialize_array(arr: &mut MaybeUninit<[i32; 100000]>) { // an uninitialized array

    unsafe {
        for i in 0..100000 {
            (*((arr.as_mut_ptr() as *mut [i32; 100000])))[i] = i as i32;
        }
        unsafe { transmute::<[i32; 100000], [i32; 100000]>(*((arr.as_mut_ptr() as *mut [i32; 100000]))) };
    }
}
