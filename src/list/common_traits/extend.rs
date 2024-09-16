use crate::{variant::Doubly, List};
use orx_selfref_col::MemoryPolicy;

impl<T, M> Extend<T> for List<Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.push_back(x);
        }
    }
}

impl<'a, T: Clone, M> Extend<&'a T> for List<Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
{
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        for x in iter {
            self.push_back(x.clone());
        }
    }
}
