use data_structures::heap_array::HeapArray;
use core::raw::Slice as RawSlice;
use std::ops::{Index, IndexMut};
use std::mem;
use std::cmp::max;
use std::iter::range_step;

static DEFAULT_CAPACITY: usize = 10us;

pub struct ArrayList<A> {
    length: usize,
    elements: HeapArray<A>,
}

impl<A> ArrayList<A> {
    pub fn new() -> ArrayList<A> {
        ArrayList {
            length: 0,
            elements: HeapArray::with_capacity(DEFAULT_CAPACITY),
        }
    }

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

    pub fn push(&mut self, element: A) {
        self.ensure_enough_capacity();

        self.elements[self.length] = element;
        self.length += 1;
    }

    pub fn insert(&mut self, index: usize, element: A) {
        if index > self.length {
            panic!(
                "index out of bounds: the index {} has to be less than the length {}",
                index,
                self.length()
            );
        }

        self.ensure_enough_capacity();

        self.length += 1;
        let previous_length = self.length - 1;
        for i in range(index, previous_length) {
            self.elements.swap(previous_length - i, previous_length - i - 1);
        }


        self.elements[index] = element;
    }

    pub fn remove_at(&mut self, index: usize) {
        if index >= self.length {
            panic!(
                "index out of bounds: the index {} has to be less than the length {}",
                index,
                self.length()
            );
        }

        for i in range(index, self.length - 1) {
            self.elements.swap(i, i + 1);
        }

        self.length -= 1;
    }

    #[inline]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [A] {
        unsafe {
            mem::transmute(RawSlice {
                data: &self.elements[0] as *const A,
                len: self.length,
            })
        }
    }

    pub fn capacity(&self) -> usize {
        self.elements.capacity()
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

impl<A> AsSlice<A> for ArrayList<A> {
    #[inline]
    fn as_slice<'a>(&'a self) -> &'a [A] {
        unsafe {
            mem::transmute(RawSlice {
                data: &self.elements[0] as *const A,
                len: self.length,
            })
        }
    }
}

impl<A> Index<usize> for ArrayList<A> {
    type Output = A;

    #[inline]
    fn index<'a>(&'a self, index: &usize) -> &'a A {
        &self.as_slice()[*index]
    }
}

impl<A> IndexMut<usize> for ArrayList<A> {
    type Output = A;

    #[inline]
    fn index_mut<'a>(&'a mut self, index: &usize) -> &'a mut A {
        &mut self.as_mut_slice()[*index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        let mut a = ArrayList::with_capacity(2);

        assert_eq!(2us, a.capacity());

        a.push(0u8);
        a.push(1u8);

        assert_eq!(2us, a.capacity());

        a.push(2u8);

        assert_eq!(4us, a.capacity());
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

        assert_eq!([1u8, 2u8, 4u8, 5u8], a.as_slice());

        a.remove_at(3);

        assert_eq!([1u8, 2u8, 4u8], a.as_slice());
    }

    #[test]
    fn insert_tests() {
        let mut a = ArrayList::with_capacity(5);
println!("1");
        a.insert(0, 5u8);

        assert_eq!(5u8, a[0]);
println!("2");
        a.insert(0, 15u8);

        assert_eq!(15u8, a[0]);

        assert_eq!(5u8, a[1]);

        assert_eq!(5us, a.capacity());
        assert_eq!(2us, a.length());
println!("3");
        a.insert(2, 1u8);

        assert_eq!(5us, a.capacity());
        assert_eq!(3us, a.length());
println!("4");
        a.insert(3, 2u8);

        assert_eq!([15u8, 5u8, 1u8, 2u8], a.as_slice());
    }

    #[test]
    #[should_fail]
    fn insert_out_of_bounds() {
        let mut a = ArrayList::with_capacity(2);
        a.insert(1, 0u8);
    }

    #[test]
    #[should_fail]
    fn remove_at_out_of_bounds() {
        let mut a: ArrayList<u8> = ArrayList::with_capacity(2);
        a.remove_at(0);
    }
}
