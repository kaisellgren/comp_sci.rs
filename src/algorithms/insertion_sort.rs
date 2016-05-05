/// Efficient sorting against small or already sorted sets.
///
/// Insertion sort is inefficient against large sets. It requires no additional memory and is stable.
///
/// It is efficient against already substantially sorted sets (`O(nk)` when each element is no more
/// than `k` places away from its sorted position).
///
/// Insertion sort can also sort sets as it receives them.
pub fn insertion_sort<A: Ord>(data: &mut [A]) {
    match data.len() {
        0 | 1 => (),
        size => {
            for i in 1 .. size {
                let mut x = i;
                while x > 0 && &data[x - 1] > &data[x] {
                    data.swap(x, x - 1);
                    x -= 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut data = vec![9, 8, 1, 5, 3, 16, 2, 0, 4];
        insertion_sort(data.as_mut_slice());
        assert_eq!([0, 1, 2, 3, 4, 5, 8, 9, 16], data.as_slice());
    }
}
