use crate::{Doubly, DoublyIterable, ListSlice, Singly, SinglyIterable};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node};

// singly

impl<T, M, P> PartialEq for ListSlice<'_, Singly<T>, M, P>
where
    T: PartialEq,
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
    fn eq(&self, other: &Self) -> bool {
        self.eq_to_iter_refs(other.iter())
    }
}

impl<T, M, P> Eq for ListSlice<'_, Singly<T>, M, P>
where
    T: PartialEq,
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
}

// doubly

impl<T, M, P> PartialEq for ListSlice<'_, Doubly<T>, M, P>
where
    T: PartialEq,
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    fn eq(&self, other: &Self) -> bool {
        self.eq_to_iter_refs(other.iter())
    }
}

impl<T, M, P> Eq for ListSlice<'_, Doubly<T>, M, P>
where
    T: PartialEq,
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
}
