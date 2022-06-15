#![allow(unused_imports)]

pub mod sorts;

use crate::sorts::{Sorter,
                   bubble::BubSort};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_works() {
        let mut arr = vec![3, 2, 1, 0];
        sorts::StdSort.sort_this(&mut arr);
        assert_eq!(arr, &[0, 1, 2, 3]);
    }
}
