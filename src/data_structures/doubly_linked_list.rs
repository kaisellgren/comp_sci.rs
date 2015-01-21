use std::mem;
use std::ptr;

/*
  DoublyLinkedList is not idiomatic to write in Rust, because of the fact that there can be two
  parties owning the same data (when the first and the last elements of the list are the same).
  For this reason, we use raw mutable pointers for ´last´ and ´previous´, instead of boxed pointers.

  We still use boxed pointers for ´first´ and ´next´ to ensure safe memory handling. Rc pointers
  could be used as well, but they entail an unnecessary performance penalty we like to avoid.
*/

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
