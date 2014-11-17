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

use std::num::Bounded;

/// Efficient sorting against small or already sorted sets.
///
/// Insertion sort is inefficient against large sets. It requires no additional memory and is stable.
///
/// It is efficient against already substantially sorted sets (`O(nk)` when each element is no more
/// than `k` places away from its sorted position).
///
/// Insertion sort can also sort sets as it receives them.
pub fn insertion_sort<'a, A: Ord + 'a>(data: &'a mut [A]) {
    let size = data.len();
    if size < 2 { return; }

    for x in range(1, size) {
        let mut x2 = x;
        while x2 > 0 && &data[x2 - 1] > &data[x2] {
            data.swap(x2, x2 - 1);
            x2 -= 1;
        }
    }
}

/// Efficient sorting against small sets.
///
/// Selection sort is inefficient against large sets. It requires no additional memory.
///
/// The write performance of `O(n)` is better than that of e.g. insertion sort's `O(n^2)`.
pub fn selection_sort<'a, A: Ord + Bounded + 'a>(data: &'a mut [A]) {
    let size = data.len();
    if size < 2 { return; }

    let mut current_min: Option<uint> = None;
    let ref max: A = Bounded::max_value();

    for x in range(0, size) {
        for x2 in range(x, size) {
            if &data[x2] < current_min.map_or(max, |current| &data[current]) {
                current_min = Some(x);
                data.swap(x, x2);
            }
        }
        current_min = None;
    }
}

    struct Slice(uint, uint);
/// Efficient sorting against large sets. Requires `O(n)` aux. space.
///
/// This divide-and-conquer sorting algorithm, while inefficient with memory use, performs
/// `O(n log n)` in average, worst and best case scenarios even against large sets of data.
pub fn merge_sort<A: Ord + Clone>(data: &[A]) -> Vec<A> {

    fn divide<A: Ord + Clone>(data: &[A], slice: Slice) -> Vec<A> {
        let Slice(begin, end) = slice;
        let data_size = end - begin;

        if data_size == 0 { return Vec::new() }
        if data_size == 1 { return vec![data[begin].clone()] }

        let middle = data_size / 2 + begin;
        let left = Slice(begin, middle);
        let right = Slice(middle, end);

        merge(merge_sort_internal(data, left), merge_sort_internal(data, right))
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

    merge_sort_internal(data, Slice(0u, data.len()))
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