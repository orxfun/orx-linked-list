use crate::{List, variant::Doubly};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node};

impl<T, M, P> Extend<T> for List<Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.push_back(x);
        }
    }
}

impl<'a, T: Clone, M, P> Extend<&'a T> for List<Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        for x in iter {
            self.push_back(x.clone());
        }
    }
}
