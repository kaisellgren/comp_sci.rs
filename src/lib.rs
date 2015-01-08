//! Various algorithms written in Rust.
//!
//! # Sorting algorithms
//!
//! Algorithm | Best case | Average case | Worst case | Space complexity
//! :-------- | :-------: | :----------: | :--------: | :-------------:
//! Selection sort | O(n^2) | O(n^2) | O(n^2) | O(n) total, O(1) aux.
//! Insertion sort | O(n) comparisons, O(1) swaps | O(n^2) comparisons, swaps | O(n^2) comparisons, swaps | O(n) total, O(1) aux.
//! Merge sort | O(n log n) | O(n log n) | O(n log n) | O(n^2) total, O(n) aux.
//! Quick sort | O(n log n) | O(n log n) | O(n^2) | O(n log n) total, O(log n) aux.
//!

#![doc(html_root_url="https://kaisellgren.github.io/doc")]
#![feature(associated_types)]

extern crate rand;
extern crate test;
extern crate core;
extern crate alloc;
extern crate serialize;

mod data_structures;
mod tests;

/// Removes duplicate entries from Vec with a complexity of O(n log n + n) I believe (TODO).
///
/// This technique sorts the vector before removing the duplicates and thus is not stable.
pub fn remove_duplicates_by_sorting<'a, A: PartialEq + Ord>(data: &'a mut Vec<A>) {
    quick_sort(data.as_mut_slice());

    let mut current_index = 0;

    while current_index < data.len() - 1 {
        if &data[current_index] == &data[current_index + 1] {
            data.remove(current_index + 1);
            continue;
        }

        current_index += 1;
    }
}

/// Removes duplicate entries from Vec with a complexity of O(n(n+1)/2).
///
/// It is based on the dual pointer technique where ´current´ iterates as usual,
/// but ´runner´ iterates until it hits the ´current´, and then ´current´ proceeds.
pub fn remove_duplicates_with_dual_pointers<'a, A: PartialEq>(data: &'a mut Vec<A>) {
    let mut current_index = 0;

    while current_index < data.len() {
        let mut runner_index = 0;
        while runner_index < current_index {
            if &data[runner_index] == &data[current_index] {
                data.remove(current_index);
                current_index -= 1;
                break;
            }

            runner_index += 1;
        }

        current_index += 1;
    }
}

/// Efficient sorting against small or already sorted sets.
///
/// Insertion sort is inefficient against large sets. It requires no additional memory and is stable.
///
/// It is efficient against already substantially sorted sets (`O(nk)` when each element is no more
/// than `k` places away from its sorted position).
///
/// Insertion sort can also sort sets as it receives them.
pub fn insertion_sort<'a, A: Ord + 'a>(data: &'a mut [A]) {
    match data.len() {
        0 | 1 => (),
        size => {
            for i in range(1, size) {
                let mut x = i;
                while x > 0 && &data[x - 1] > &data[x] {
                    data.swap(x, x - 1);
                    x -= 1;
                }
            }
        }
    }
}

/// Efficient sorting against small sets.
///
/// Selection sort is inefficient against large sets. It requires no additional memory.
///
/// The write performance of `O(n)` is better than that of e.g. insertion sort's `O(n^2)`.
pub fn selection_sort<'a, A: Ord + 'a>(data: &'a mut [A]) {
    let (mut i, size) = (0, data.len());

    while i < size {
        let (mut x, mut current_min) = (i + 1, i);
        while x < size {
            if data[x] < data[current_min] {
                current_min = x;
            }
            x += 1;
        }
        data.swap(i, current_min);
        i += 1;
    }
}

/// Efficient sorting against large sets. Requires `O(n)` aux. space.
///
/// This divide-and-conquer sorting algorithm, while inefficient with memory use, performs
/// `O(n log n)` in average, worst and best case scenarios even against large sets of data.
pub fn merge_sort<A: Ord + Clone>(data: &[A]) -> Vec<A> {
    fn divide<A: Ord + Clone>(data: &[A]) -> Vec<A> {
        match data.len() {
            0 => vec![],
            1 => vec![data[0].clone()],
            size => {
                let middle = size / 2;
                conquer(divide(data.slice_to(middle)), divide(data.slice_from(middle)))
            }
        }
    }

    fn conquer<A: Ord + Clone>(left: Vec<A>, right: Vec<A>) -> Vec<A> {
        let mut left_index = 0;
        let mut right_index = 0;

        let left_size = left.len();
        let right_size = right.len();

        let mut result = Vec::with_capacity(left_size + right_size);

        while left_index < left_size && right_index < right_size {
            if left[left_index] < right[right_index] {
                result.push(left[left_index].clone());
                left_index += 1;
            } else {
                result.push(right[right_index].clone());
                right_index += 1;
            }
        }

        result.push_all(left.slice_from(left_index));
        result.push_all(right.slice_from(right_index));

        result
    }

    divide(data)
}

/// Efficient sorting against large sets.
///
/// This divide-and-conquer sorting algorithm performs `O(n log n)` in average and best case scenarios,
/// but `O(n^2)` in the worst case.
///
/// Quicksort is often faster in practice than other `O(n log n)` algorithms due to sequential and
/// localized memory references that work well with modern CPU caches.
pub fn quick_sort<A: Ord>(data: &mut [A]) {
    fn qsort<A: Ord>(data: &mut [A]) {
        match data.len() {
            0 | 1 => (),
            _ => {
                let pivot = find_pivot(data);
                let pivot = partition(data, pivot);

                qsort(data.slice_to_mut(pivot));
                qsort(data.slice_from_mut(pivot + 1));
            }
        }
    }

    /// Partitioning makes the left values of the pivot to be less, and the right values to be greater.
    fn partition<A: Ord>(data: &mut [A], pivot: uint) -> uint {
        let (mut next_position, mut index) = (0, 0);
        let right_index = data.len() - 1;

        data.swap(pivot, right_index);

        while index < right_index {
            if data[index] <= data[right_index] {
                data.swap(index, next_position);
                next_position += 1;
            }

            index += 1;
        }

        data.swap(next_position, right_index);
        next_position
    }

    /// Finds the median of left, middle and right.
    fn find_pivot<A: Ord>(data: &[A]) -> uint {
        let (left, right) = (0, data.len() - 1);
        let middle = left + (right - left) / 2; // Avoid integer overflow vs (left + right) / 2.

        match (&data[left], &data[middle], &data[right]) {
            (l, m, r) if l <= m && m <= r => middle,
            (l, m, r) if l >= m && l <= r => left,
            _ => right,
        }
    }

    qsort(data)
}
