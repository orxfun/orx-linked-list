use crate::{DoublyIterable, List, Singly, SinglyIterable, variant::Doubly};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node};

use super::from_iter::{doubly_from_iter, singly_from_iter};

impl<T: Clone, M, P> Clone for List<Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>> + Default,
{
    fn clone(&self) -> Self {
        singly_from_iter(self.iter().cloned())
    }
}

impl<T: Clone, M, P> Clone for List<Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>> + Default,
{
    fn clone(&self) -> Self {
        doubly_from_iter(self.iter().cloned())
    }
}
