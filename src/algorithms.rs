pub fn insertion_sort<'a, T: Ord + 'a>(data: &'a mut [T]) {
    let size = data.len();
    if size < 2 { return; }

    for x in range(1, size) {
        let mut x2 = x;
        while x2 > 0 && &data[x2 - 1] > &data[x2] {
            data.swap(x2, x2 - 1);
            x2 -= 1;
        }
    }
}