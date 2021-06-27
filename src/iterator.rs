use crate::heap::HeapPermutor;
use crate::Permutable;
use crate::Permutor;

/// Iterator over the permutations of a permutable type
pub struct PermuteIter<T: Permutable>{
    p: HeapPermutor,
    source: T,
}


#[cfg(feature = "grapheme")]
use crate::permutable::GraphemeString;

/// Creates a string iterator that iterates over graphemes rather than
/// codepoints
#[cfg(feature = "grapheme")]
impl PermuteIter<String> {
    pub fn with_graphemes(&self) -> PermuteIter<GraphemeString> {
        PermuteIter {
            p: HeapPermutor::new(self.len()),
            source: GraphemeString::new(self),
        }
    }
}

impl<T> Iterator for PermuteIter<T>
where T: Permutable + Clone
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.p.finished() {
            None
        } else {
            let val = self.source.clone();
            self.p.permute(&mut self.source);
            Some(val)
        }
    }
}

/// Create permute iter from a permutable type
impl<T: Permutable> From<T> for PermuteIter<T> {
    fn from(source: T) -> Self {
        PermuteIter {
            p: HeapPermutor::new(source.length()),
            source,
        }
    }
}

/// PermuteIter from string slice
impl From<&str> for PermuteIter<String> {
    fn from(string: &str) -> Self {
        PermuteIter {
            p: HeapPermutor::new(string.len()),
            source: string.into(),
        }
    }
}
