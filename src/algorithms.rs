//! Various algorithms written in Rust.
//!
//! # Sorting algorithms
//!
//! Algorithm | Best case | Average case | Worst case | Space complexity
//! :-------- | :-------: | :----------: | :--------: | :-------------:
//! Selection sort | O(n^2) | O(n^2) | O(n^2) | O(n) total, O(1) aux.
//! Insertion sort | O(n) comparisons, O(1) swaps | O(n^2) comparisons, swaps | O(n^2) comparisons, swaps | O(n) total, O(1) aux.
//!

#![doc(html_root_url="https://kaisellgren.github.io/doc")]

use std::num::Bounded;

pub fn insertion_sort<'a, T: Ord + 'a>(data: &'a mut [T]) {
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

pub fn selection_sort<'a, T: Ord + Bounded + 'a>(data: &'a mut [T]) {
    let size = data.len();
    if size < 2 { return; }

    let mut current_min: Option<uint> = None;
    let ref max: T = Bounded::max_value();

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

pub fn merge_sort<T: Ord + Clone>(data: &[T]) -> Vec<T> {
    struct Slice(uint, uint);

    fn merge_sort_internal<T: Ord + Clone>(data: &[T], slice: Slice) -> Vec<T> {
        let Slice(begin, end) = slice;
        let data_size = end - begin;

        if data_size == 0 { return Vec::new() }
        if data_size == 1 { return vec![data[begin].clone()] }

        let middle = data_size / 2 + begin;
        let left = Slice(begin, middle);
        let right = Slice(middle, end);

        merge(merge_sort_internal(data, left), merge_sort_internal(data, right))
    }

    fn merge<T: Ord + Clone>(left: Vec<T>, right: Vec<T>) -> Vec<T> {
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