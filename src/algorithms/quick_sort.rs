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
    fn partition<A: Ord>(data: &mut [A], pivot: usize) -> usize {
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
    fn find_pivot<A: Ord>(data: &[A]) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut data = vec![9, 8, 1, 5, 3, 16, 2, 0, 4];
        quick_sort(data.as_mut_slice());
        assert_eq!([0, 1, 2, 3, 4, 5, 8, 9, 16], data.as_slice());
    }
}
