//! Collection of types that can be permuted by the permutor

/// Represents any type that the heap algorithm can generate permutations for
pub trait Permutable {
    fn switch(&mut self, a: usize, b: usize);
    fn length(&self) -> usize;
}

/// Permutable for generic vec
impl<T> Permutable for Vec<T> {
    #[inline]
    fn switch(&mut self, a: usize, b: usize) {
        self.swap(a, b);
    }

    #[inline]
    fn length(&self) -> usize {
        self.len()
    }
}

/// Permutable for String
impl Permutable for String {
    fn switch(&mut self, a: usize, b: usize) {
        unsafe {
            self.as_mut_vec().swap(a, b);
        }   
    }

    #[inline]
    fn length(&self) -> usize {
        self.len()
    }
}

/// The normal Permutable String permutes codepoints in addition to "regular"
/// characters. GraphemeString is a string wrapper that allows the permutation
/// of the graphemes that make up the string rather than all the codepoints.
#[cfg(feature = "grapheme")]
#[repr(transparent)]
pub struct GraphemeString {
    inner: Vec<&str>,
}

#[cfg(feature = "grapheme")]
impl GraphemeString {
    fn new(string: String) -> GraphemeString {
        GraphemeString {
            inner: string.graphemes(true).collect::<Vec<&str>>(),
        }
    }
}

/// Permutable for Grapheme String
#[cfg(feature = "grapheme")]
impl Permutable for GraphemeString {
    fn switch(&mut self, a: usize, b: usize) {
        self.inner.swap(a, b);
    }

    #[inline]
    fn length(&self) -> usize {
        self.inner.len()
    }
}

// Implement permutable for the primitive numeric rust types
macro_rules! primitive_numeric {
    ($p:ty) => {
        impl Permutable for $p {
            #[inline]
            fn switch(&mut self, a: usize, b: usize) {
                let len = self.length();

                if !(a < len && b < len) {
                    panic!("Could not switch bits on primitive, out of range");
                }

                if (((*self & (1 << a)) >> a) ^ ((*self & (1 << b)) >> b)) != 0 {
                    *self ^= 1 << a;
                    *self ^= 1 << b;
                }
            }

            #[inline]
            fn length(&self) -> usize {
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