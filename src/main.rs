extern crate rand;

use rand::{thread_rng, Rng};
use std::time::Instant;

fn main() {
    // list lengths
    let sizes: [usize; 3] = [1000, 10_000, 100_000];
    let random_samples = 1;
    let mut arr = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    merge_sort(&mut arr);
    println!("{:?}", arr);
    for size in &sizes {
        let vals = (0..(*size)).map(|_| thread_rng().gen()).collect::<Vec<_>>();
        println!("bubble: {}", measure(&vals, &mut bubble_sort));
        println!("insertion: {}", measure(&vals, &mut insertion_sort));
    }
}

fn bubble_sort(arr: &mut [usize]) {
    for j in 0..(arr.len() - 1) {
        for i in 1..(arr.len() - j) {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
            }
        }
    }
}

fn merge_sort(arr: &mut Vec<usize>) {
    mergesort(arr, 0, arr.len()-1)
}

pub fn mergesort(arr: &mut Vec<usize>, b: usize, e: usize) {
    if b < e {
        let m = (b+e)/2;
        mergesort(arr, b, m);
        mergesort(arr, m+1, e);
        merge(arr, b, m, e);
    }
}
fn merge(arr: &mut Vec<usize>, b: usize, m:usize, e:usize) {
    let mut left = arr[b..m+1].to_vec();
    let mut right = arr[m+1..e+1].to_vec();
    left.reverse();
    right.reverse();
    for k in b..e + 1 {
        if left.is_empty() {
            arr[k] = right.pop().unwrap();
            continue;
        }
        if right.is_empty() {
            arr[k] = left.pop().unwrap();
            continue;
        }
        if right.last() < left.last() {
            arr[k] = right.pop().unwrap();
        }
        else {
            arr[k] = left.pop().unwrap();
        }
    }
}

fn counting_sort(arr: &mut [usize]) {
    let mut max = 0;
    for i in 0..arr.len() {
        if arr[i] > max {
            max = arr[i];
        }
    }
    let mut buckets = (0..max).map(|_| 0).collect::<Vec<_>>();
    for i in 0..arr.len() {
        buckets[arr[i] - 1] += 1;
    }
    let mut i = 0;
    for j in 0..arr.len() {
        while buckets[i] == 0 {
            i += 1;
        }
        buckets[i] -= 1;
        arr[j] = i+1;
    }
}

fn heap_sort(arr: &mut [usize]) {
    heapsort(arr, arr.len() as isize)
}

fn heapsort(arr: &mut [usize], n: isize) {
    // Build heap (rearrange array)
    let mut i = n / 2 - 1;
    while i >= 0 {
        heapify(arr, n as usize, i as usize);
        i -= 1;
    }

    // One by one extract an element from heap
    i = n - 1;
    while i > 0 {
        // Move current root to end
        arr.swap(0, i as usize);

        // call max heapify on the reduced heap
        heapify(arr, i as usize, 0);
        i -= 1;
    }
}

fn heapify(arr: &mut [usize], n: usize, i: usize) {
    let mut largest = i; // Initialize largest as root
    let l = 2 * i + 1; // left = 2*i + 1
    let r = 2 * i + 2; // right = 2*i + 2

    // If left child is larger than root
    if l < n && arr[l] > arr[largest] {
        largest = l;
    }

    // If right child is larger than largest so far
    if r < n && arr[r] > arr[largest] {
        largest = r;
    }

    // If largest is not root
    if largest != i {
        arr.swap(i, largest);

        // Recursively heapify the affected sub-tree
        heapify(arr, n, largest);
    }
}

fn selection_sort(arr: &mut [usize]) {
    for i in 0..(arr.len() - 1) {
        let mut x = arr[i];
        let mut k = i;
        for j in (i + 1)..arr.len() {
            if arr[j] < x {
                x = arr[j];
                k = j;
            }
        }
        arr[k] = arr[i];
        arr[i] = x;
    }
}

fn shell_sort(arr: &mut [usize]) {
    let mut column_count = [
        2147483647, 1131376761, 410151271, 157840433, 58548857, 21521774, 8810089, 3501671,
        1355339, 543749, 213331, 84801, 27901, 11969, 4711, 1968, 815, 271, 111, 41, 13, 4, 1,
    ];
    for k in 0..column_count.len() {
        let h = column_count[k];
        for i in h..arr.len() {
            let t = arr[i];
            let mut j = i;
            while j >= h && arr[j - h] > t {
                arr[j] = arr[j - h];
                j = j - h;
            }
            arr[j] = t;
        }
    }
}

fn quick_sort(arr: &mut [usize]) {
    let len = arr.len() as isize;
    quicksort(arr, 0, len - 1)
}

fn quicksort(arr: &mut [usize], lo: isize, hi: isize) {
    if lo < hi {
        let p = partition(arr, lo, hi);
        quicksort(arr, lo, p - 1);
        quicksort(arr, p + 1, hi);
    }
}

fn partition(arr: &mut [usize], lo: isize, hi: isize) -> isize {
    let pivot = arr[hi as usize];
    let mut i = lo;
    for j in lo..hi {
        if arr[j as usize] < pivot {
            arr.swap(i as usize, j as usize);
            i += 1;
        }
    }
    arr.swap(i as usize, hi as usize);
    i
}

fn shaker_sort(arr: &mut [usize]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..(arr.len() - 2) {
            if arr[i] > arr[i + 1] {
                swapped = true;
                arr.swap(i, i + 1);
            }
        }
        if !swapped {
            return;
        }
        swapped = false;
        for j in 0..(arr.len() - 2) {
            let i = (arr.len() - 2) - j;
            if arr[i] > arr[i + 1] {
                swapped = true;
                arr.swap(i, i + 1);
            }
        }
    }
}

fn insertion_sort(arr: &mut [usize]) {
    for i in 1..arr.len() {
        let mut j = (i - 1) as isize;
        let x = arr[i];
        while j >= 0 && arr[j as usize] > x {
            j -= 1;
        }
        let mut k = i;
        while k >= (j + 2) as usize {
            arr[k] = arr[k - 1];
            k -= 1;
        }
        arr[(j + 1) as usize] = x;
    }
}

fn measure(arr: &Vec<usize>, sort: &mut FnMut(&mut [usize])) -> u128 {
    let mut arr = arr.clone();
    let now = Instant::now();
    sort(&mut arr);
    now.elapsed().as_nanos()
}

fn invert(arr: &mut [usize]) {
    let half = arr.len() / 2;
    for i in 0..half {
        arr.swap(i, arr.len() - i - 1);
    }
}
