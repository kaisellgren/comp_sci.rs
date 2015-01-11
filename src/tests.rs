use insertion_sort;
use quick_sort;
use merge_sort;
use selection_sort;
use rand::XorShiftRng;
use rand::Rng;
use test::Bencher;
use test::black_box;
use remove_duplicates_with_dual_pointers;
use remove_duplicates_by_sorting;

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
    let mut data = vec![9, 8, 1, 5, 3, 16, 2, 0, 4];
    data = merge_sort(data.as_mut_slice());
    assert_sorted(data.as_slice());
}

#[test]
fn test_quick_sort() {
    let mut data = vec![9, 8, 1, 5, 3, 16, 2, 0, 4];
    quick_sort(data.as_mut_slice());
    assert_sorted(data.as_slice());
}

#[test]
fn test_remove_duplicates_with_dual_pointers() {
    let mut v = vec![1u32, 2, 3, 4, 5, 4, 3, 2, 1, 0];
    remove_duplicates_with_dual_pointers(&mut v);
    assert_eq!(v, vec![1u32, 2, 3, 4, 5, 0]);
}

#[test]
fn test_remove_duplicates_by_sorting() {
    let mut v = vec![1u32, 2, 3, 4, 5, 4, 3, 2, 1, 0];
    remove_duplicates_by_sorting(&mut v);
    assert_eq!(v, vec![0u32, 1, 2, 3, 4, 5]);
}

/*#[bench]
fn basic(b: &mut Bencher) {
    let mut data = data();
    b.iter(|| {
        range(0u8, 5).collect::<Vec<u8>>();
    });
    println!("s: {}", b.ns_per_iter() / 1000000);
}*/
