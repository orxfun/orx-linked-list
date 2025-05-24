use crate::{
    Doubly, Singly,
    list::helper_traits::{HasCol, HasDoublyEnds, HasSinglyEnds},
    type_aliases::{DefaultMemory, DefaultPinVec},
    variant::ListVariant,
};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node, SelfRefCol, Variant};

/// A slice of a linked list.
///
/// Note that a list slice itself behaves pretty much like a linked list.
/// However, it does not own the data, but provides a view on it, just as a slice of a vec.
pub struct ListSlice<'a, V, M = DefaultMemory<V>, P = DefaultPinVec<V>>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
    P: PinnedVec<Node<V>>,
{
    pub(crate) col: &'a SelfRefCol<V, M, P>,
    pub(crate) ends: V::Ends,
}

impl<V, M, P> HasCol<V, M, P> for ListSlice<'_, V, M, P>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
    P: PinnedVec<Node<V>>,
{
    #[inline(always)]
    fn col(&self) -> &SelfRefCol<V, M, P> {
        self.col
    }
}

impl<T, M, P> HasSinglyEnds<T, M, P> for ListSlice<'_, Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
    #[inline(always)]
    fn ends(&self) -> &<Singly<T> as Variant>::Ends {
        &self.ends
    }
}

impl<T, M, P> HasDoublyEnds<T, M, P> for ListSlice<'_, Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    #[inline(always)]
    fn ends(&self) -> &<Doubly<T> as Variant>::Ends {
        &self.ends
    }
}
