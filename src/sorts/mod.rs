#![allow(unused_imports)]
pub mod bubble;
pub mod insertion;
pub mod quick;

pub use self::{bubble::BubSort,
               insertion::InsertionSort,
               quick::QuickSort};

pub trait Sorter<T: Ord> {
    fn sort_this(&self, list: &mut [T]);
}

pub struct StdSort;

impl<T: Ord> Sorter<T> for StdSort {
    fn sort_this(&self, list: &mut [T]) {
        list.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_sorted() {
        let mut arr = vec![0, 3, 2, 1];
        quick::QuickSort.sort_this(&mut arr);
        assert_eq!(arr, &[0, 1, 2, 3]);
    }
}
