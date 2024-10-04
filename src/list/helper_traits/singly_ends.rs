use super::{HasCol, HasColMut};
use crate::Singly;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node, Variant};

/// Lists and views with owned ends.
pub trait HasSinglyEnds<T, M, P>: HasCol<Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
    /// Returns a reference to the ends of the linked list.
    fn ends(&self) -> &<Singly<T> as Variant>::Ends;
}

/// Lists and views with owned mutable ends.
pub trait HasSinglyEndsMut<T, M, P>: HasColMut<Singly<T>, M, P> + HasSinglyEnds<T, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
    /// Returns a mutable reference to the ends of the linked list.
    fn ends_mut(&mut self) -> &mut <Singly<T> as Variant>::Ends;
}
