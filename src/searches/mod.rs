pub mod binary;

pub use self::binary::binary_search;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_found() {
        let position = binary_search(&vec![0, 1, 2, 3, 4, 5], &2);
        assert_eq!(Some(2), position);
    }
}
