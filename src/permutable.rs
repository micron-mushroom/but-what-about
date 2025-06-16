//! Collection of types that can be permuted by the permutor
use unicode_segmentation::UnicodeSegmentation;
use core::ops::Range;

/// Represents any type that the we can generate permutations for.
/// By default `str` is handled as `[u8]`, which can result in invalid
/// UTF-8 if you try to permute a `str` with non-ascii characters. If you
/// need to deal with Graphemes use the GraphemeString instead.
pub trait Permutable {
    fn swap(&mut self, a: usize, b: usize);
    fn len(&self) -> usize;
}

impl<T> Permutable for [T] {
    fn swap(&mut self, a: usize, b: usize) {
        <[T]>::swap(self, a, b);
    }

    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

impl Permutable for str {
    fn swap(&mut self, a: usize, b: usize) {
        unsafe {
            <[u8] as Permutable>::swap(self.as_bytes_mut(), a, b);
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl Permutable for &mut str {
    fn swap(&mut self, a: usize, b: usize) {
        <str as Permutable>::swap(*self, a, b);
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}



/// The normal Permutable String permutes codepoints in addition to "regular"
/// characters. GraphemeString is a string wrapper that allows the permutation
/// of the graphemes that make up the string rather than all the codepoints.
pub struct GraphemeString {
    inner: Box<[u8]>,
    length: usize,
    buf: Vec<u8>, // TinyVec might be more appropriate here
}

impl From<&str> for GraphemeString {
    fn from(value: &str) -> GraphemeString {
        GraphemeString {
            inner: Box::from(value.as_bytes()),
            length: UnicodeSegmentation::graphemes(value, true).count(),
            buf: Vec::with_capacity(3),
        }
    }
}


/// Permutable for Grapheme String
impl Permutable for GraphemeString {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= self.length || b >= self.length {
            panic!("Items out of range, could not swap {} with {}. The container length is {}", a, b, self.length);
        }

        // This is redundant but explicit.
        if a == b || self.inner.len() < 2 || self.length < 2 {
            return;
        }

        let (a, b) = (usize::min(a, b), usize::max(a, b));

        // What the ranges of the bytes of the ath and bth grapheme?
        let mut a_range = 0..0;
        let mut b_range = 0..0;
        let mut idx = 0;

        for (start, string) in UnicodeSegmentation::grapheme_indices(self.as_ref(), true) {
            match idx {
                n if n == a => a_range = start..start + self.len(),
                n if n == b => b_range = start..start + self.len(),
                _ => {}
            }
            idx += 1;
        }

        // We will swap the smaller range first, because the larger is guaranteed to accomodate it.
        let order = a_range.count().cmp(&b_range.count());
        use std::cmp::Ordering::*;
        
        let (min_range, max_range, min_length, max_length) = match order {
            Less | Equal => (a_range, b_range, a_range.count(), b_range.count()),
            Greater => (b_range, a_range, b_range.count(), a_range.count())
        };

        // Copy the larger range into memory.
        self.buf.clear();
        self.inner[max_range].iter().collect_into(&mut self.buf);
        
        // Swap min(len(a), len(b)) common items
        std::mem::swap(&mut self.inner[min_range], &mut self.inner[max_range.start..max_range.start + min_length]);

        // Shift items that were in middle (and some copied ones when shifting right)
        let shift_by = max_length - min_length; 

        unsafe {
            match order {
                Equal => return,
                Less => std::ptr::copy(),
                Greater => std::ptr::copy(),
            }
        }

        // Finish copying larger item. 

    }

    #[inline]
    fn len(&self) -> usize {
        self.length
    }
}

impl AsRef<str> for GraphemeString {
    fn as_ref(&self) -> &str {
        std::str::from_utf8(&self.inner).unwrap()
    }
}

// Implement permutable for the primitive numeric rust types. Kindof useless, but still fun.
macro_rules! primitive_numeric {
    ($p:ty) => {
        impl Permutable for $p {
            #[inline]
            fn swap(&mut self, a: usize, b: usize) {
                let len = <$p as Permutable>::len(self);

                if !(a < len && b < len) {
                    panic!("Could not switch bits on primitive, out of range");
                }

                if (((*self & (1 << a)) >> a) ^ ((*self & (1 << b)) >> b)) != 0 {
                    *self ^= 1 << a;
                    *self ^= 1 << b;
                }
            }

            #[inline]
            fn len(&self) -> usize {
                std::mem::size_of::<$p>() * 8
            }
        }
    };
}

primitive_numeric!(i8);
primitive_numeric!(i16);
primitive_numeric!(i32);
primitive_numeric!(i64);
primitive_numeric!(i128);
primitive_numeric!(isize);

primitive_numeric!(u8);
primitive_numeric!(u16);
primitive_numeric!(u32);
primitive_numeric!(u64);
primitive_numeric!(u128);
primitive_numeric!(usize);
