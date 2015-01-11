use core::nonzero::NonZero;
use core::raw::Slice as RawSlice;
use alloc::heap::EMPTY;
use alloc::heap::allocate;
use std::usize;
use std::ptr::copy_nonoverlapping_memory;
use std::mem;
use std::ptr;
use std::num::Int;
use std::ops::{Index, IndexMut};

/// An implementation of a fixed-size mutable array, which is allocated on the heap.
///
/// It has a complexity of O(1) for indexing.
pub struct HeapArray<A> {
    pointer: NonZero<*mut A>,
    capacity: usize,
}

impl<A> HeapArray<A> {
    #[inline]
    pub fn with_capacity(capacity: usize) -> HeapArray<A> {
        let a_size = mem::size_of::<A>();

        if a_size == 0 {
            HeapArray {
                pointer: unsafe {
                    NonZero::new(EMPTY as *mut A)
                },
                capacity: usize::MAX, // Empty sized A's yield infinite capacity.
            }
        } else if capacity == 0 {
            HeapArray {
                pointer: unsafe {
                    NonZero::new(EMPTY as *mut A)
                },
                capacity: 0,
            }
        } else {
            let bytes = capacity.checked_mul(a_size).expect("capacity overflow");
            let pointer = unsafe {
                allocate(bytes, mem::min_align_of::<A>())
            };

            if pointer.is_null() { ::alloc::oom() }

            HeapArray {
                pointer: unsafe {
                    NonZero::new(pointer as *mut A)
                },
                capacity: capacity,
            }
        }
    }

    #[inline]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [A] {
        unsafe {
            mem::transmute(RawSlice {
                data: *self.pointer as *const A,
                len: self.capacity,
            })
        }
    }

    /// Creates a new array with the given capacity and copies the contents to it.
    pub fn copy(&self, capacity: usize) -> HeapArray<A> {
        let mut new_array = HeapArray::with_capacity(capacity);
        unsafe {
            copy_nonoverlapping_memory(&mut new_array[0], &self[0], self.capacity);
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

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<A> AsSlice<A> for HeapArray<A> {
    #[inline]
    fn as_slice<'a>(&'a self) -> &'a [A] {
        unsafe {
            mem::transmute(RawSlice {
                data: *self.pointer as *const A,
                len: self.capacity,
            })
        }
    }
}

impl<A> Index<usize> for HeapArray<A> {
    type Output = A;

    #[inline]
    fn index<'a>(&'a self, index: &usize) -> &'a A {
        &self.as_slice()[*index]
    }
}

impl<A> IndexMut<usize> for HeapArray<A> {
    type Output = A;

    #[inline]
    fn index_mut<'a>(&'a mut self, index: &usize) -> &'a mut A {
        &mut self.as_mut_slice()[*index]
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
    assert_eq!(0u8, b[2]);
    assert_eq!(0u8, b[3]);

    assert_eq!(6u8, a[0]);
    assert_eq!(11u8, a[1]);

    assert_eq!(4, b.capacity());
}
