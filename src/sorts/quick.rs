use crate::sorts::Sorter;

pub struct QuickSort;

fn partition<T: Ord>(list: &mut [T]) -> usize {
    let end = list.len() - 1;
    let mut j = 0;
    for i in 0..end {
        if list[j] <= list[end] {
            list.swap(i, j);
            j += 1;
        }
    }
    list.swap(end, j);
    j
}

fn quick_sort<T: Ord>(list: &mut [T]) {
    if !list.is_empty() {
        let j = partition(list);
        let len = list.len();

        quick_sort(&mut list[0..j]);
        quick_sort(&mut list[j+1..len]);
    }
}

impl<T: Ord> Sorter<T> for QuickSort {
    fn sort_this(&self, list: &mut [T]) {
        quick_sort(list);
    }
}
