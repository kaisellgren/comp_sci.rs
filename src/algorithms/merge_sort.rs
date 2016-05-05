/// Efficient sorting against large sets. Requires `O(n)` aux. space.
///
/// This divide-and-conquer sorting algorithm, while inefficient with memory use, performs
/// `O(n log n)` in average, worst and best case scenarios even against large sets of data.
pub fn merge_sort<A: Ord + Clone>(data: &[A]) -> Vec<A> {
    fn divide<A: Ord + Clone>(data: &[A]) -> Vec<A> {
        match data.len() {
            0 => vec![],
            1 => vec![data[0].clone()],
            size => {
                let middle = size / 2;
                conquer(divide(&data[..middle]), divide(&data[middle..]))
            }
        }
    }

    fn conquer<A: Ord + Clone>(left: Vec<A>, right: Vec<A>) -> Vec<A> {
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

        result.extend_from_slice(&left[left_index..]);
        result.extend_from_slice(&right[right_index..]);

        result
    }

    divide(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut data = vec![9, 8, 1, 5, 3, 16, 2, 0, 4];
        data = merge_sort(data.as_mut_slice());
        assert_eq!([0, 1, 2, 3, 4, 5, 8, 9, 16], data.as_slice());
    }
}
