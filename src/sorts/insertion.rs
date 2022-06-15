use crate::sorts::Sorter;

pub struct InsertionSort;

impl<T: Ord> Sorter<T> for InsertionSort {
    fn sort_this(&self, list: &mut [T]) {
        for i in 1..list.len() {
            let mut j = i;
            while j > 0 && list[j - 1] > list[j] {
                list.swap(j - 1, j);
                j -= 1;
            }
        }
    }
}
