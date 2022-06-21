use std::cmp::Ordering;

pub fn binary_search<T: Ord>(list: &[T], x: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match x.cmp(&list[mid]) {
            Ordering::Less => high = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => low = mid + 1,
        }
    }
    None
}
