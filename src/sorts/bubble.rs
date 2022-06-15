use crate::sorts::Sorter;

pub struct BubSort;

impl<T: Ord> Sorter<T> for BubSort {
    fn sort_this(&self, list: &mut [T]) {
        let mut sorted = false;
        while !sorted {
            sorted = true;
            for i in 0..(list.len() - 1) {
                if list[i] > list[i + 1] {
                    list.swap(i, i + 1);
                    sorted = false;
                }
            }
        }
    }
}
