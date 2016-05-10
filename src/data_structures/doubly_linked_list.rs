use std::mem::align_of;
use std::mem::size_of;
use std::mem::transmute;
use std::ptr;
use alloc::heap::deallocate;
use alloc::heap::allocate;

struct Node<A> {
    prev: *mut Node<A>,
    next: *mut Node<A>,
    value: A
}

// push_back, push_front, iter, clear
pub struct DoublyLinkedList<A> {
    first: *mut Node<A>,
    last: *mut Node<A>,
    length: usize
}

fn convert<'a, A>(value: *mut Node<A>) -> Option<&'a A> {
    unsafe {
        value.as_ref().map(|node| &node.value)
    }
}

impl<A> DoublyLinkedList<A> {
    pub fn new() -> DoublyLinkedList<A> {
        DoublyLinkedList {
            first: ptr::null_mut(),
            last: ptr::null_mut(),
            length: 0
        }
    }

    pub fn clear(&mut self) {
        let mut next = self.first;
        unsafe {
            while let Some(node) = next.as_ref() {
                let prev = next;
                next = node.next;
                let bytes = size_of::<A>() + size_of::<usize>() * 2;
                println!("to dealloc: {:?}", bytes);
                deallocate(prev as *mut u8, bytes, align_of::<A>());
            }
        }
// TODO: CALL DROP()? transmute back to Box?
        self.first = ptr::null_mut();
        self.last = ptr::null_mut();
        self.length = 0;
    }

    pub fn append(&mut self, value: A) {
        let previous_last = self.last;

        

        let mut node = Box::new(Node {
            prev: previous_last,
            next: ptr::null_mut(),
            value: value
        });

        if self.is_empty() {
            println!("eh");
            self.first = &mut *node;
        }

        self.last = &mut *node;
        self.length += 1;

        unsafe {
            if let Some(prev) = previous_last.as_mut() {
                prev.next = self.last;
            }
        }
    }

    pub fn first(&self) -> Option<&A> {
        convert(self.first)
    }

    pub fn last(&self) -> Option<&A> {
        convert(self.last)
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Node;
    use std::mem;

    fn node_value<'a, A>(pointer: *mut Node<A>) -> &'a A {
        unsafe {
            &pointer.as_ref().unwrap().value
        }
    }

    #[test]
    fn basic() {
        let mut list: DoublyLinkedList<u8> = DoublyLinkedList::new();

        assert!(list.first.is_null());
        assert!(list.last.is_null());

        list.append(5);

        assert_eq!(list.len(), 1);
        assert_eq!(*node_value(list.first), 5);
        assert_eq!(*node_value(list.last), 5);
        println!("first: {:?}, last: {:?}", list.first, list.last);

        list.append(10);

        println!("first: {:?}, last: {:?}", list.first, list.last);
        assert_eq!(list.len(), 2);
        assert_eq!(*node_value(list.first), 5);
        assert_eq!(*node_value(list.last), 10);

        list.append(15);

        assert_eq!(list.len(), 3);
        assert_eq!(*node_value(list.first), 5);
        assert_eq!(*node_value(list.last), 15);

        //list.clear();
    }
}


























/*
/// An implementation of a doubly linked list.
pub struct DoublyLinkedList<A> {
    first: Option<Box<Node<A>>>,
    last: *mut Node<A>,
    length: usize,
}

impl<A> DoublyLinkedList<A> {
    #[inline]
    pub fn new() -> DoublyLinkedList<A> {
        DoublyLinkedList {
            first: None,
            last: ptr::null_mut::<Node<A>>(),
            length: 0,
        }
    }

    #[inline]
    pub fn push_front(&mut self, value: A) {
        let mut new_node = Box::new(Node::new(value));

        match self.first.take() {
            Some(mut first) => {
                first.previous = &mut *new_node;
                mem::swap(&mut first, &mut new_node);
                first.next = Some(new_node);
                self.first = Some(first);
            },
            None => {
                debug_assert!(self.last.is_null());

                self.last = &mut *new_node;
                self.first = Some(new_node);
            },
        }

        self.length += 1;
    }

    #[inline]
    pub fn push_back(&mut self, value: A) {
        let last: Option<&mut Node<A>> = unsafe {
            mem::transmute(self.last.as_mut())
        };

        match last {
            Some(last) => {
                let mut new_node = Box::new(Node::new(value));

                self.last = &mut *new_node;
                new_node.previous = last;
                last.next = Some(new_node);
                self.length += 1;
            },
            None => self.push_front(value),
        }
    }

    /// Provides a forward iterator.
    #[inline]
    pub fn iter(&self) -> Iter<A> {
        Iter {
            length: self.length(),
            first: &self.first,
            last: self.last
        }
    }

    /// Retrieves the first element.
    #[inline]
    pub fn first(&self) -> Option<&A> {
        self.first.as_ref().map(|node| &node.value)
    }

    /// Retrieves the last element.
    #[inline]
    pub fn last(&self) -> Option<&A> {
        let value: Option<&Node<A>> = unsafe {
            mem::transmute(self.last.as_ref())
        };

        value.map(|node| &node.value)
    }

    /// Returns whether there are any elements in the list.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.first.is_some()
    }

    /// Returns the length for this list.
    #[inline]
    pub fn length(&self) -> usize {
        self.length
    }

    /// Removes all elements from the list.
    ///
    /// This operation is `O(n)`.
    #[inline]
    pub fn clear(&mut self) {
        *self = DoublyLinkedList::new()
    }
}

struct Node<A> {
    next: Option<Box<Node<A>>>,
    previous: *mut Node<A>,
    value: A,
}

impl<A> Node<A> {
    #[inline]
    pub fn new(value: A) -> Node<A> {
        Node {
            next: None,
            previous: ptr::null_mut::<Node<A>>(),
            value: value,
        }
    }
}

/// An iterator over the references to the items of a DoublyLinkedList.
pub struct Iter<'a, A: 'a> {
    first: &'a Option<Box<Node<A>>>,
    last: *mut Node<A>,
    length: usize,
}

impl<'a, A> Iterator for Iter<'a, A> {
    type Item = &'a A;

    #[inline]
    fn next(&mut self) -> Option<&'a A> {
        if self.length == 0 {
            None
        } else {
            self.first.as_ref().map(|first| {
                self.length -= 1;
                self.first = &first.next;
                &first.value
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.length, Some(self.length))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn basic_tests() {
        let mut list = DoublyLinkedList::new();

        list.push_front(5u32);

        assert_eq!(5u32, *list.first().unwrap());
        assert_eq!(5u32, *list.last().unwrap());

        list.push_front(10u32);

        assert_eq!(10u32, *list.first().unwrap());
        assert_eq!(5u32, *list.last().unwrap());

        list.push_back(15u32);

        assert_eq!(10u32, *list.first().unwrap());
        assert_eq!(15u32, *list.last().unwrap());
    }

    #[test]
    fn more_tests() {
        let mut list = DoublyLinkedList::new();

        assert!(list.first().is_none());
        assert!(list.last().is_none());
        assert_eq!(0, list.length());

        list.push_front(3u32);

        assert_eq!(3u32, *list.first().unwrap());
        assert_eq!(3u32, *list.last().unwrap());
        assert_eq!(1, list.length());

        list.push_front(1u32);

        assert_eq!(1u32, *list.first().unwrap());
        assert_eq!(3u32, *list.last().unwrap());
    }

    #[test]
    fn iter() {
        let mut list = DoublyLinkedList::new();

        list.push_front(10u32);
        list.push_front(10u32);
        list.push_front(10u32);

        for _ in list.iter() {}
    }
}
*/
