# but-what-about
The crate is a permutation utility that only supports Heap's algorithm. At the moment it can permute strings(by grapheme and codepoint&mdash;though, codepoint based permutation is highly questionable at the moment) and primitive numeric types by their bits. In the future I plan to add support for additional permutation algorithms and implement some combination algorithms as well. I can't say I recomment using the crate for anything more than just fun at the moment. So, do go and have fun.

```rust 
use heap_permute::PermuteIter;
// Print all of the permutations of a string of three characters
fn main() {
    const STRING: &'static str = "ABC";
    for p in PermuteIter::from(STRING) {
        print!("{}, ", p);
    }
}
// Prints: 
// ABC, BAC, CAB, ACB, BCA, CBA,
```