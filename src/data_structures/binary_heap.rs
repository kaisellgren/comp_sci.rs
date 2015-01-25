/// An implementation of a binary heap.
///
/// This is a max-heap implementation.
pub struct BinaryHeap<A: Ord> {
    data: Vec<A>,
}

/// Helpers for iterating the heap.
mod index {
    pub fn parent(i: usize) -> usize { (i - 1) / 2 }
    pub fn first_child(i: usize) -> usize { 2 * i + 1 }
    pub fn second_child(i: usize) -> usize { 2 * i + 2 }
}

impl<A: Ord> BinaryHeap<A> {
    #[inline]
    pub fn new() -> BinaryHeap<A> {
        BinaryHeap { data: Vec::new() }
    }

    /// Pushes a new element into the heap.
    #[inline]
    pub fn push(&mut self, element: A) {
        let length = self.data.len();
        self.data.push(element);
        self.sift_up(0, length);
    }

    /// Pops out the most important element.
    #[inline]
    pub fn pop(&mut self) -> Option<A> {
        match self.data.len() {
            0 => None,
            1 => self.data.pop(),
            length => {
                let last = length - 1;
                self.data.swap(0, last);
                let value = Some(self.data.remove(last));
                if last > 1 {
                    self.sift_down(0, last - 1);
                }
                value
            },
        }
    }

    #[inline]
    fn sift_up(&mut self, top_index: usize, current_index: usize) {
        if top_index != current_index {
            let parent_index = index::parent(current_index);

            if self.data[parent_index] < self.data[current_index] {
                self.data.swap(parent_index, current_index);
                self.sift_up(top_index, parent_index);
            }
        }
    }

    #[inline]
    fn sift_down(&mut self, current_index: usize, bottom_index: usize) {
        let first_index = index::first_child(current_index);

        if first_index != bottom_index {
            if self.data[current_index] < self.data[first_index] {
                self.data.swap(current_index, first_index);
                self.sift_down(first_index, bottom_index);
            } else {
                let second_index = index::second_child(current_index);
                let has_children = second_index != bottom_index;

                if has_children && self.data[current_index] < self.data[second_index] {
                    self.data.swap(current_index, second_index);
                    self.sift_down(second_index, bottom_index);
                }
            }
        }
    }

    /// Returns the length of this heap.
    pub fn length(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn basic_tests() {
        let mut bh = BinaryHeap::new();
        bh.push(5u8);
        bh.push(15u8);
        bh.push(10u8);

        assert_eq!(Some(15u8), bh.pop());
        assert_eq!(Some(10u8), bh.pop());
        assert_eq!(Some(5u8), bh.pop());
        assert_eq!(None, bh.pop());
    }

    #[bench]
    fn pushing(b: &mut Bencher) {
        b.iter(|| {
            let mut bh = BinaryHeap::new();

            for i in range(0u32, 1_000) {
                bh.push(i);
            }
        })
    }
}
