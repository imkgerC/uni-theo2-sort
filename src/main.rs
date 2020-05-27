extern crate rand;

use rand::{thread_rng, Rng};
use std::time::Instant;

fn main() {
    // list lengths
    let sizes: [usize; 3] = [1000, 10_000, 100_000];
    let random_samples = 1;
    for size in &sizes {
        let vals = (0..(*size)).map(|_| thread_rng().gen()).collect::<Vec<_>>();
        println!("{}", measure(&vals, &mut bubble_sort));
    }
}

fn bubble_sort(arr: &mut [usize]) {
    let mut swaps = 1;
    while swaps > 0 {
        swaps = 0;
        for i in 1..arr.len() {
            if arr[i-1] > arr[i] {
                arr.swap(i-1, i);
                swaps += 1;
            }
        }
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