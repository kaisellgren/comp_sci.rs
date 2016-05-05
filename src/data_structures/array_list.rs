use data_structures::heap_array::HeapArray;
use std::slice::from_raw_parts;
use std::slice::from_raw_parts_mut;
use std::ops::{Index, IndexMut};
use std::mem;
use std::cmp::max;
use std::convert::AsRef;

static DEFAULT_CAPACITY: usize = 10usize;

/// An implementation of a growable and mutable array type, which is allocated on the heap.
///
/// Minimum memory requirement is three pointer sized integers, e.g. 24-bytes on 64-bit system.
pub struct ArrayList<A> {
    length: usize,
    elements: HeapArray<A>,
}

impl<A> ArrayList<A> {
    /// Creates a new array list with a default capacity of 10.
    pub fn new() -> ArrayList<A> {
        ArrayList {
            length: 0,
            elements: HeapArray::with_capacity(DEFAULT_CAPACITY),
        }
    }

    /// Creates a new array list with the given capacity.
    pub fn with_capacity(capacity: usize) -> ArrayList<A> {
        ArrayList {
            length: 0,
            elements: HeapArray::with_capacity(capacity),
        }
    }

    fn ensure_enough_capacity(&mut self) {
        if self.length == self.capacity() {
            self.elements = self.elements.copy(self.capacity() * 2);
        }
    }

    /// Adds an element to the end of the list.
    ///
    /// More capacity will be allocated if necessary.
    pub fn push(&mut self, element: A) {
        self.ensure_enough_capacity();

        self.elements[self.length] = element;
        self.length += 1;
    }

    /// Inserts an element at the given index.
    ///
    /// More capacity will be allocated if necessary.
    pub fn insert(&mut self, index: usize, element: A) {
        if index > self.length {
            panic!(
                "index out of bounds: the index {} has to be less than the length {}",
                index,
                self.length()
            );
        }

        self.ensure_enough_capacity();

        for i in index .. self.length {
            self.elements.swap(self.length - i, self.length - i - 1);
        }

        self.elements[index] = element;
        self.length += 1;
    }

    /// Removes an element at the given index.
    pub fn remove_at(&mut self, index: usize) {
        if index >= self.length {
            panic!(
                "index out of bounds: the index {} has to be less than the length {}",
                index,
                self.length()
            );
        }

        for i in index .. self.length - 1 {
            self.elements.swap(i, i + 1);
        }

        self.length -= 1;
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [A] {
        unsafe {
            from_raw_parts_mut(&mut self.elements[0], self.length)
        }
    }

    /// Returns the capacity of this list.
    pub fn capacity(&self) -> usize {
        self.elements.capacity()
    }

    /// Returns the number of elements in the list.
    pub fn length(&self) -> usize {
        self.length
    }
}

impl<A> AsRef<[A]> for ArrayList<A> {
    fn as_ref(&self) -> &[A] {
        unsafe {
            from_raw_parts(&self.elements[0], self.length)
        }
    }
}

impl<A> AsMut<[A]> for ArrayList<A> {
    fn as_mut(&mut self) -> &mut [A] {
        unsafe {
            from_raw_parts_mut(&mut self.elements[0], self.length)
        }
    }
}

impl<A> Index<usize> for ArrayList<A> {
    type Output = A;

    fn index(&self, index: usize) -> &A {
        &self.as_ref()[index]
    }
}

impl<A> IndexMut<usize> for ArrayList<A> {
    fn index_mut(&mut self, index: usize) -> &mut A {
        &mut self.as_mut()[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        let mut a = ArrayList::with_capacity(2);

        assert_eq!(2usize, a.capacity());

        a.push(0u8);
        a.push(1u8);

        assert_eq!(2usize, a.capacity());

        a.push(2u8);

        assert_eq!(4usize, a.capacity());
    }

    #[test]
    fn remove_at_tests() {
        let mut a = ArrayList::with_capacity(5);

        a.push(1u8);
        a.push(2u8);
        a.push(3u8);
        a.push(4u8);
        a.push(5u8);

        a.remove_at(2);

        assert_eq!([1u8, 2u8, 4u8, 5u8], a.as_ref());

        a.remove_at(3);

        assert_eq!([1u8, 2u8, 4u8], a.as_ref());
    }

    #[test]
    fn insert_tests() {
        let mut a = ArrayList::with_capacity(5);

        a.insert(0, 5u8);

        assert_eq!(5u8, a[0]);

        a.insert(0, 15u8);

        assert_eq!(15u8, a[0]);

        assert_eq!(5u8, a[1]);

        assert_eq!(5usize, a.capacity());
        assert_eq!(2usize, a.length());

        a.insert(2, 1u8);

        assert_eq!(5usize, a.capacity());
        assert_eq!(3usize, a.length());

        a.insert(3, 2u8);

        assert_eq!([15u8, 5u8, 1u8, 2u8], a.as_ref());
    }

    #[test]
    #[should_panic]
    fn insert_out_of_bounds() {
        let mut a = ArrayList::with_capacity(2);
        a.insert(1, 0u8);
    }

    #[test]
    #[should_panic]
    fn remove_at_out_of_bounds() {
        let mut a: ArrayList<u8> = ArrayList::with_capacity(2);
        a.remove_at(0);
    }
}
