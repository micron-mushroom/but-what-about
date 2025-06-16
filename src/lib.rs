#![doc = include_str!("../README.md")]
#![feature(iter_collect_into)]

mod heap;
mod iterator;
mod permutable;

pub use crate::iterator::PermuteIter;
pub use crate::permutable::Permutable;

pub use crate::permutable::GraphemeString;

/// A type that can permute a permutable, in this crate there is only one at the
/// moment.
pub trait Permutor<'a, T: Permutable + 'a>: From<&'a T> {
    fn permute(&mut self, source: &mut T);
    fn finished(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn permute_heap<T>(val: T, result: &mut Vec<T>, bound: usize)
    where
        T: Permutable + Clone + std::fmt::Debug + std::cmp::PartialEq,
    {
        for p in PermuteIter::<heap::HeapPermutor, T>::from(val).take(bound) {
            let cmp = result.pop().unwrap();
            // Output
            eprintln!("{:?}(Permutor), {:?}(Required)", p, cmp);
            assert_eq!(p, cmp);
        }
    }


    #[test]
    fn slice_and_str() {
        let mut result: Vec<&str> = ["ABC", "BAC", "CAB", "ACB", "BCA", "CBA"]
            .iter()
            .copied()
            .rev()
            .collect();

        eprintln!("Permutations for test(length): {}", result.len());
        // Check that at least the number of results is equal to 3!
        
        let mut val = String::from("ABC");
        permute_heap(&mut val, &mut result, 6);
    }

    #[test]
    fn bits() {
        let mut result = vec![
            0b10101001, 0b10101010, 0b10101010, 0b10101001, 0b10101100, 0b10101100, 0b10100101,
            0b10100110, 0b10100011, 0b10100011,
        ]
        .iter()
        .rev()
        .cloned()
        .collect::<Vec<u8>>();

        permute_heap(0b1010_1001, &mut result, 10);
    }

    #[test]
    fn grapheme_string() {
        let base = "g̈a";
        let graphemes = GraphemeString::from(base);
        assert_eq!(graphemes.as_ref(), base);
    }
}
