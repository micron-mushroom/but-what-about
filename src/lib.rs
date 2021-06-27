//! The crate is basically just a permutation utility. With the goal of speed
//! and the smallest possible runtime memory impact possible. And so, the goal
//! of this crate is to be able to calculate and obtain a permutation of a value
//! as soon as possible.
//!
//! # Examples
//! Get all the permutations of a string of three characters:
//! ```rust
//! use heap_permute::PermuteIter;
//! 
//! // Print all of the permutations of a string of three characters
//! fn main() {
//!     const STRING: &'static str = "ABC";
//!
//!     for p in PermuteIter::from(STRING) {
//!         println!("{}", p);
//!     }
//! }
//! // Prints: 
//! // ABC
//! // BAC
//! // CAB
//! // ACB
//! // BCA
//! // CBA
//! ```
//!
//! Now time for something more interesting, let's permute the bits in a u8:
//! ```rust
//! use heap_permute::PermuteIter;
//! 
//! fn main() {
//!     for p in PermuteIter::from(0b1010_1001 as u8) {
//!         println!("0b{:08b} ({})", p, p);
//!     }
//! }
//! // Prints:
//! // 0b10101001 (169)
//! // 0b10101010 (170)
//! // 0b10101010 (170)
//! // 0b10101001 (169)
//! // 0b10101100 (172)
//! // 0b10101100 (172)
//! // 0b10100101 (165)
//! // 0b10100110 (166)
//! // 0b10100011 (163)
//! // 0b10100011 (163)
//! // ...
//! ```
//!
//! As it happens, the crate supports permutation for all of the primitive
//! numeric rust types.
//!

mod permutable;
mod heap;
mod iterator;

pub use crate::permutable::Permutable;
pub use crate::iterator::PermuteIter;
#[cfg(feature = "grapheme")]
pub use crate::permutable::GraphemeString;

/// A type that can permute a permutable, in this crate there is only one at the
/// moment.
trait Permutor {
    fn permute(&mut self, source: &mut impl Permutable);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn permute_general<T>(val: T, result: &mut Vec<T>, bound: usize)
    where T: Permutable + Clone + std::fmt::Debug + std::cmp::PartialEq
    {
        for p in PermuteIter::from(val).take(bound) {
            let cmp = result.pop().unwrap();
            // Output
            eprintln!("{:?}(Permutor), {:?}(Required)", p, cmp);
            assert_eq!(p, cmp);
        }
    }

    #[test]
    fn permute_four() {
        let string: String = "ABCD".into();

        let mut result: Vec<String> = vec![
            "ABCD",
            "BACD",
            "CABD",
            "ACBD",
            "BCAD",
            "CBAD",
            "DBCA",
            "BDCA",
            "CDBA",
            "DCBA",
            "BCDA",
            "CBDA",
            "DACB",
            "ADCB",
            "CDAB",
            "DCAB",
            "ACDB",
            "CADB",
            "DABC",
            "ADBC",
            "BDAC",
            "DBAC",
            "ABDC",
            "BADC",
        ].iter().map(|s| String::from(*s)).rev().collect();

        eprintln!("Permutations for test(length): {}", result.len());
        // Check that at least the number of results is equal to 4!
        assert!(result.len() == 24);

        permute_general(string, &mut result, 24);
    }

    #[test]
    fn permute_three() {
        let string: String = "ABC".into();

        let mut result: Vec<String> = vec![
            "ABC",
            "BAC",
            "CAB",
            "ACB",
            "BCA",
            "CBA",
        ].iter().map(|s| String::from(*s)).rev().collect();

        eprintln!("Permutations for test(length): {}", result.len());
        // Check that at least the number of results is equal to 3!
        assert!(result.len() == 6);

        permute_general(string, &mut result, 6);
    }

    #[test]
    fn permute_bits() {
        let mut result = vec![
            0b10101001,
            0b10101010,
            0b10101010,
            0b10101001,
            0b10101100,
            0b10101100,
            0b10100101,
            0b10100110,
            0b10100011,
            0b10100011,
        ].iter().rev().cloned().collect::<Vec<u8>>();

        permute_general(0b1010_1001, &mut result, 10);
    }
}