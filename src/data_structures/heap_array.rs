use std::slice::from_raw_parts_mut;
use std::slice::from_raw_parts;
use alloc::heap::allocate;
use std::usize;
use std::ptr::copy_nonoverlapping;
use std::mem;
use std::ptr;
use std::ops::{Index, IndexMut};

/// An implementation of a fixed-size mutable array, which is allocated on the heap.
///
/// Minimum memory requirement is two pointer sized integers, e.g. 16-bytes on 64-bit system.
pub struct HeapArray<A> {
    pointer: *mut A,
    capacity: usize,
}

impl<A> HeapArray<A> {
    /// Creates a new HeapArray by allocating the given amount of capacity.
    ///
    /// There is no way to increase or decrease capacity afterwards.
    #[inline]
    pub fn with_capacity(capacity: usize) -> HeapArray<A> {
        let a_size = mem::size_of::<A>();

        if a_size == 0 {
            HeapArray {
                pointer: 0 as *mut A,
                capacity: usize::MAX, // Empty sized A's yield infinite capacity.
            }
        } else if capacity == 0 {
            HeapArray {
                pointer: 0 as *mut A,
                capacity: 0,
            }
        } else {
            let bytes = capacity.checked_mul(a_size).expect("capacity overflow");
            let pointer = unsafe {
                allocate(bytes, mem::align_of::<A>())
            };

            if pointer.is_null() { ::alloc::oom() }

            HeapArray {
                pointer: pointer as *mut A,
                capacity: capacity,
            }
        }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [A] {
        unsafe {
            from_raw_parts_mut(self.pointer, self.capacity)
        }
    }

    /// Creates a new array with the given capacity and copies the contents to it.
    pub fn copy(&self, capacity: usize) -> HeapArray<A> {
        let mut new_array = HeapArray::with_capacity(capacity);
        unsafe {
            copy_nonoverlapping(&self[0], &mut new_array[0], self.capacity);
        }
        new_array
    }

    /// Swaps the elements at given indices.
    pub fn swap(&mut self, a: usize, b: usize) {
        unsafe {
            let ptr_a = self.pointer.offset(a as isize);
            let ptr_b = self.pointer.offset(b as isize);
            ptr::swap(ptr_a, ptr_b);
        }
    }

    /// Returns the capacity for this array.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<A> AsRef<[A]> for HeapArray<A> {
    #[inline]
    fn as_ref(&self) -> &[A] {
        unsafe {
            from_raw_parts(self.pointer, self.capacity)
        }
    }
}

impl<A> Index<usize> for HeapArray<A> {
    type Output = A;

    #[inline]
    fn index(&self, index: usize) -> &A {
        &self.as_ref()[index]
    }
}

impl<A> IndexMut<usize> for HeapArray<A> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut A {
        &mut self.as_mut_slice()[index]
    }
}

#[test]
fn basic_tests() {
    let mut a = HeapArray::with_capacity(10);

    assert_eq!(10, a.capacity());

    a[0] = 5u8;
    a[1] = 10u8;

    assert_eq!(15, a[0] + a[1]);

    // Modify the memory directly and see if the array returns what we expect.
    unsafe {
        let ptr: *mut u8 = mem::transmute(&(a[0]));
        *(ptr.offset(2)) = 20u8;
    }

    assert_eq!(20u8, a[2]);
}

#[test]
fn copy_test() {
    let mut a = HeapArray::with_capacity(2);

    a[0] = 5u8;
    a[1] = 10u8;

    let b = a.copy(4);

    assert_eq!(5u8, a[0]);
    assert_eq!(10u8, a[1]);

    a[0] = 6u8;
    a[1] = 11u8;

    assert_eq!(5u8, b[0]);
    assert_eq!(10u8, b[1]);

    assert_eq!(6u8, a[0]);
    assert_eq!(11u8, a[1]);

    assert_eq!(4, b.capacity());
}
