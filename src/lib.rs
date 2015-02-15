//! This project consists of articles, data structures, algorithms and solutions to common problems.
//!
//! # Sorting algorithms
//!
//! Algorithm | Best case | Average case | Worst case | Auxiliary space
//! :-------- | :-------: | :----------: | :--------: | :-------------:
//! Selection sort | O(n^2) | O(n^2) | O(n^2) | O(1)
//! Insertion sort | O(n) | O(n^2) | O(n^2) | O(1)
//! Merge sort | O(n log n) | O(n log n) | O(n log n) | O(n)
//! Quick sort | O(n log n) | O(n log n) | O(n^2) | O(log n)
//!
//! # Data structures
//!
//! Always choose your data structures carefully. Look below for guidance.
//!
//! ### Indexing performance
//!
//! Indexing is the act of accessing an arbitrary element in the data structure.
//! This is usually achieved with `[]` in Rust.
//!
//! Data structure | Average case | Worst case
//! :------------: | :----------: | :--------:
//! HeapArray | O(1) | O(1)
//! ArrayList | O(1) | O(1)
//!
//! ### Search performance
//!
//! Searching is the act of finding an arbitrary element within the data structure.
//! This is usually achieved with a method such as `contains()`, `get()` or similar.
//!
//! Data structure | Average case | Worst case
//! :------------: | :----------: | :--------:
//! HeapArray | O(n) | O(n)
//! ArrayList | O(n) | O(n)
//!
//! ### Insertion performance
//!
//! Insertion is the act of inserting an element at an arbitrary position within the data structure.
//! This is usually achieved with a method such as `insert()`, and there is sometimes a `push()`
//! for inserting at the end of the data structure given it makes sense.
//!
//! Data structure | Best case | Average case | Amortized | Worst case
//! :------------: | :-------: | :----------: | :-------: | :--------:
//! HeapArray[1] | N/A | N/A | N/A | N/A
//! ArrayList[2] | O(1) | O(n) | O(n - index) | O(n)
//!
//! [1]: HeapArray is fixed-size thus this function is not available.
//!
//! [2]: ArrayList's insertion performance depends on the index you add an element to.
//! Adding to the end of the list is `O(1)` while adding to the front is `O(n)`, because all the prior elements would have to be moved forward.
//!
//! Further more, if the capacity of the list is exceeded, it will be `O(n)` as the entire list has to be reallocated.
//!
//! ### Deletion performance
//!
//! Deletion is the act of removing an element from the data structure. This is usually achieved with
//! a method such as `remove()`.
//!
//! Data structure | Best case | Average case | Amortized | Worst case
//! :------------: | :-------: | :----------: | :-------: | :--------:
//! HeapArray[1] | N/A | N/A | N/A | N/A
//! ArrayList[2] | O(1) | O(n) | O(n - index) | O(n)
//!
//! [1]: HeapArray is fixed-size thus this function is not available.
//!
//! [2]: ArrayList's deletion performance depends on the index you delete an element from. Deleting from the end of the list is `O(1)` while deleting from the front is `O(n)`, because all the prior elements would have to be moved backward.
//!
//! ### Space complexity
//!
//! Space complexity defines how much memory is necessary to represent the data structure.
//!
//! Data structure | Space complexity
//! :------------: | :----------:
//! HeapArray | O(n)
//! ArrayList | O(n)

#![doc(html_root_url="https://kaisellgren.github.io/doc")]
#![allow(unstable)]
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate rand;
extern crate test;
extern crate core;
extern crate alloc;
extern crate serialize;

pub mod algorithms;
pub mod data_structures;

fn assert_sorted<T: Ord>(data: &[T]) {
    let mut index = 1;
    while index < data.len() {
        assert!(data[index - 1] <= data[index]);
        index += 1;
    }
}

/// Removes duplicate entries from Vec with a complexity of O(n log n + n) I believe (TODO).
///
/// This technique sorts the vector before removing the duplicates and thus is not stable.
pub fn remove_duplicates_by_sorting<'a, A: PartialEq + Ord>(data: &'a mut Vec<A>) {
    algorithms::quick_sort::quick_sort(data.as_mut_slice());

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

/// Finds the position of the key within the given slice.
///
/// This is a O(log n) on average and at worst, O(1) at best.
/// Note: the given slice must be ordered.
pub fn binary_search<A: Ord>(data: &[A], key: A) -> usize {
    fn accumulator<A: Ord>(data: &[A], key: A, offset: usize) -> usize {
        let middle = data.len() / 2;

        if data[middle] > key {
            accumulator(data.slice_to(middle), key, offset)
        } else if data[middle] < key {
            accumulator(data.slice_from(middle), key, offset + middle)
        } else {
            middle + offset
        }
    };

    accumulator(data, key, 0)
}

#[test]
fn test_binary_search() {
    let stuff = vec![0u8, 2, 4, 6, 8, 9, 10];
    assert_eq!(1, binary_search(stuff.as_slice(), 2u8));
}

#[test]
fn test_remove_duplicates_with_dual_pointers() {
    let mut v = vec![1u32, 2, 3, 4, 5, 4, 3, 2, 1, 0];
    remove_duplicates_with_dual_pointers(&mut v);
    assert_eq!(v, vec![1u32, 2, 3, 4, 5, 0]);
}

#[test]
fn test_remove_duplicates_by_sorting() {
    let mut v = vec![1u32, 2, 3, 4, 5, 4, 3, 2, 1, 0];
    remove_duplicates_by_sorting(&mut v);
    assert_eq!(v, vec![0u32, 1, 2, 3, 4, 5]);
}
