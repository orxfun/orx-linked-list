use crate::{Doubly, DoublyIterable, ListSlice, Singly, SinglyIterable};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node};

// singly

impl<'a, T, M, P> PartialEq for ListSlice<'a, Singly<T>, M, P>
where
    T: PartialEq,
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
    fn eq(&self, other: &Self) -> bool {
        self.eq_to_iter_refs(other.iter())
    }
}

impl<'a, T, M, P> Eq for ListSlice<'a, Singly<T>, M, P>
where
    T: PartialEq,
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
}

// doubly

impl<'a, T, M, P> PartialEq for ListSlice<'a, Doubly<T>, M, P>
where
    T: PartialEq,
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    fn eq(&self, other: &Self) -> bool {
        self.eq_to_iter_refs(other.iter())
    }
}

impl<'a, T, M, P> Eq for ListSlice<'a, Doubly<T>, M, P>
where
    T: PartialEq,
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
}
