//! The crate is basically just a permutation utility. With the goal of speed
//! and the smallest possible runtime memory impact possible. And so, the goal
//! of this crate is to be able to calculate and obtain a permutation of a value
//! as soon as possible.
//! 
//! # Examples
//! ```rust
//! // Print all of the permutations of a string of three characters
//! fn main() {
//!     const STRING: &'static str = "ABC";
//! 
//!     for p in PermuterIter::from(STRING) {
//!         if let Ok(string) = std::str::from_utf8(&p) {
//!             println!("{}", string);
//!         }
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

/// Permutator. I don't recommend that you use this interface for obtaining the
/// permutations because in order to make the permute function as fast as
/// possible, safety was thrown away. That being said, HeapPermutor can be
/// faster than the iterator because it isn't required to create a new Vec for
/// each yield. In practice it's about a 0.5ns speed gain and extra
/// responsibility. So unless you need it, I can't recommend it.
pub struct HeapPermutor {
    finished: bool,
    index: usize,
    stack: Vec<usize>,
}

impl HeapPermutor {
    /// Create a new heap permutor. The stack size argument should be the size
    /// of the Vec that's going to be permuted.
    pub fn new<'b>(size: usize) -> HeapPermutor {
        HeapPermutor {
            finished: false,
            index: 1,
            stack: Vec::with_capacity(size),
        }
    }

    /// Using permute on a Vec not of length size(from HeapPermutor::new) will
    /// result in undefined behaviour.
    /// # Contract
    ///  - Never modify the struct field values unless you know what you're doing
    ///  - Ensure that the `source.len()` is equal to `self.stack.len()`
    /// 
    pub unsafe fn permute<T>(&mut self, source: &mut Vec<T>) {
        let stack = self.stack.as_mut_ptr();

        while self.index < source.len() {
            if *stack.add(self.index) < self.index {
                // Swap based on index parity
                if self.index % 2 == 0 {
                    source.swap(0, self.index);
                } else {
                    source.swap(*stack.add(self.index), self.index);
                }
                
                // Increment loop counter
                *stack.add(self.index) += 1;

                // "Simulate recursive call reaching the base case by bringing the pointer to the base case analog in the array"
                self.index = 1;

                return;
            } else {
                // Loop terminated, reset state and simulate stack pop
                *stack.add(self.index) = 0;
                self.index += 1;
            }
        }

        self.finished = true;
    }

    #[inline]
    pub fn finished(&self) -> bool {
        self.finished
    }
}

/// Iterator over 
pub struct PermuteIter<T>{
    p: HeapPermutor,
    source: Vec<T>,
}

impl<T> Iterator for PermuteIter<T>
where T: Clone + 'static
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.p.finished {
            None
        } else {
            let val = self.source.clone();

            unsafe {
                self.p.permute(&mut self.source);
            }
    
            Some(val)
        }
    }
}

impl From<&'static str> for PermuteIter<u8> {
    fn from(string: &'static str) -> Self {
        PermuteIter {
            p: HeapPermutor::new(string.len()),
            source: string.bytes().collect::<Vec<u8>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    fn permute_general(string: &'static str, result: &mut Vec<&'static str>) {
        for p in super::PermuteIter::from(string) {
            // Get
            let string = std::str::from_utf8(&p[..]).unwrap();
            let cmp = result.pop().unwrap();
            // Output
            eprintln!("{}(Permutor), {}(Required)", string, cmp);
            
            assert_eq!(string, cmp);
        }
    }

    #[test]
    fn permute_four() {
        const STRING: &'static str = "ABCD";

        let mut result: Vec<&'static str> = vec![
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
        ].iter().cloned().rev().collect();

        eprintln!("Permutations for test(length): {}", result.len());
        // Check that at least the number of results is equal to 4!
        assert!(result.len() == 24);

        permute_general(STRING, &mut result);
    }

    #[test]
    fn permute_three() {
        const STRING: &'static str = "ABC";

        let mut result: Vec<&'static str> = vec![
            "ABC",
            "BAC",
            "CAB",
            "ACB",
            "BCA",
            "CBA",
        ].iter().cloned().rev().collect();

        eprintln!("Permutations for test(length): {}", result.len());
        // Check that at least the number of results is equal to 3!
        assert!(result.len() == 6);

        permute_general(STRING, &mut result);
    }
}