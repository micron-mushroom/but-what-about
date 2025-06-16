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

    #[inline]
    pub fn finished(&self) -> bool {
        self.finished
    }
}

use crate::{Permutable, Permutor};

impl<T: Permutable> From<&T> for HeapPermutor {
    fn from(value: &T) -> Self {
        Self {
            finished: false,
            index: 1,
            stack: Vec::with_capacity(<T as Permutable>::len(value)),
        }
    }
}

impl<'a, T: Permutable + 'a> Permutor<'a, T> for HeapPermutor {
    fn permute(&mut self, source: &mut T) {
        let stack = self.stack.as_mut_ptr();

        while self.index < source.len() {
            unsafe {
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
        }

        // We managed to reach the end of the function, looping is done, and we have completed a cycle of permutation
        self.finished = true;
    }

    fn finished(&self) -> bool {
        self.finished
    }
}
