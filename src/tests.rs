use insertion_sort;
use quick_sort;
use merge_sort;
use selection_sort;
use rand::XorShiftRng;
use rand::Rng;
use test::Bencher;
use test::black_box;

fn data() -> Vec<u8> {
    let mut rng = XorShiftRng::new_unseeded();
    rng.gen_iter::<u8>().take(100).collect()
}

fn assert_sorted<T: Ord>(data: &[T]) {
    let mut index = 1;
    while index < data.len() {
        assert!(data[index - 1] <= data[index]);
        index += 1;
    }
}

#[test]
fn test_selection_sort() {
    let mut data = data();
    selection_sort(data.as_mut_slice());
    assert_sorted(data.as_slice());
}

#[test]
fn test_insertion_sort() {
    let mut data = data();
    insertion_sort(data.as_mut_slice());
    assert_sorted(data.as_slice());
}

#[test]
fn test_merge_sort() {
    let mut data = data();
    data = vec![9, 8, 1, 5, 3, 16, 2, 0, 4];
    data = merge_sort(data.as_mut_slice());
    assert_sorted(data.as_slice());
}

#[test]
fn test_quick_sort() {
    let mut data = data();
    data = vec![9, 8, 1, 5, 3, 16, 2, 0, 4];
    quick_sort(data.as_mut_slice());
    assert_sorted(data.as_slice());
}