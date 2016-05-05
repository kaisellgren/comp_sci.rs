/// Efficient sorting against small sets.
///
/// Selection sort is inefficient against large sets. It requires no additional memory.
///
/// The write performance of `O(n)` is better than that of e.g. insertion sort's `O(n^2)`.
pub fn selection_sort<A: Ord>(data: &mut [A]) {
    let (mut i, size) = (0, data.len());

    while i < size {
        let (mut x, mut current_min) = (i + 1, i);
        while x < size {
            if data[x] < data[current_min] {
                current_min = x;
            }
            x += 1;
        }
        data.swap(i, current_min);
        i += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut data = vec![9, 8, 1, 5, 3, 16, 2, 0, 4];
        selection_sort(data.as_mut_slice());
        assert_eq!([0, 1, 2, 3, 4, 5, 8, 9, 16], data.as_slice());
    }
}
