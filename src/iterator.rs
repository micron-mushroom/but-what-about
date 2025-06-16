use crate::Permutable;
use crate::Permutor;

/// Iterator over the permutations of a permutable type
pub struct PermuteIter<A: for <'a> Permutor<'a, B>, B: Permutable> {
    p: A,
    source: B,
}

impl<A, B> Iterator for PermuteIter<A, B>
where
    A: for <'a> Permutor<'a, B>,
    B: Permutable + Clone,
{
    type Item = B;

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

/// Create `PermuteIter` from a permutable type.
impl<A, B> From<B> for PermuteIter<A, B>
where
    A: for <'a> Permutor<'a, B>,
    B: Permutable + Clone,
{
    fn from(source: B) -> Self {
        PermuteIter {
            p: A::from(source.clone()),
            source,
        }
    }
}
