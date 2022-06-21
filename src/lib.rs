#![allow(unused_imports)]

pub mod sorts;
pub mod searches;

use crate::sorts::{Sorter,
                   bubble::BubSort,
                   insertion::InsertionSort,
                   quick::QuickSort};

use crate::searches::binary::binary_search;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_sorts() {
        let mut arr = vec![3, 2, 1, 0];
        sorts::StdSort.sort_this(&mut arr);
        assert_eq!(arr, &[0, 1, 2, 3]);
    }
}
