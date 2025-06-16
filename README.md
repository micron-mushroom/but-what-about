# but-what-about
The crate is a permutation utility that only supports Heap's algorithm. At the moment it can permute strings(by grapheme and codepoint&mdash;though, codepoint based permutation is highly questionable at the moment) and primitive numeric types by their bits. In the future I plan to add support for additional permutation algorithms and implement some combination algorithms as well. I don't recommend using the crate for anything more than just fun at the moment. Have fun!
Ideally though things are still fast and memory efficient. Remember that Heap's algorithm doesn't account for
duplicates in the sequence being permuted.

# Examples
Get all the permutations of a string of three characters:
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
Keep in mind that the `str`s are permuted as if they were `[u8]`. If you want to permute
a string that contains non-ascii you probably want to be making the PermuteIter with a
`GraphemeString`. Which in the above amounts to adding the appropriate `use` and wrapping
the `&str` in a `GraphemeString::from()`.

You can also go through the permutations of the bits in a numeric type. Which could
be usefull with a Permutor that produces distinct permutations, unfortunatly you would
need to implement that yourself since all this crate provides is a `HeapPermutor`.
```rust
use heap_permute::PermuteIter;

fn main() {
    for p in PermuteIter::from(0b1010_1001 as u8) {
        println!("0b{:08b} ({})", p, p);
    }
}
// Prints (non-distinct):
// 0b10101001 (169)
// 0b10101010 (170)
// 0b10101010 (170)
// 0b10101001 (169)
// 0b10101100 (172)
// 0b10101100 (172)
// 0b10100101 (165)
// 0b10100110 (166)
// 0b10100011 (163)
// 0b10100011 (163)
// ...
```
