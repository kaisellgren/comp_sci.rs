use std::num::Bounded;

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

pub fn selection_sort<'a, T: Ord + Bounded + 'a>(data: &'a mut [T]) {
    let size = data.len();
    if size < 2 { return; }

    let mut current_min: Option<uint> = None;
    let ref max: T = Bounded::max_value();

    for x in range(0, size) {
        for x2 in range(x, size) {
            if &data[x2] < current_min.map_or(max, |current| &data[current]) {
                current_min = Some(x);
                data.swap(x, x2);
            }
        }
        current_min = None;
    }
}
}