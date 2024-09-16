use crate::{
    list::helper_traits::{HasCol, HasDoublyEnds, HasSinglyEnds},
    type_aliases::{DefaultMemory, PinVec},
    variant::ListVariant,
    Doubly, Singly,
};
use orx_selfref_col::{MemoryPolicy, SelfRefCol, Variant};

/// A slice of a linked list.
///
/// Note that a list slice itself behaves pretty much like a linked list.
/// However, it does not own the data, but provides a view on it, just as a slice of a vec.
pub struct ListSlice<'a, V, M = DefaultMemory<V>>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
{
    pub(crate) col: &'a SelfRefCol<V, M, PinVec<V>>,
    pub(crate) ends: V::Ends,
}

impl<'a, V, M> HasCol<V, M> for ListSlice<'a, V, M>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
{
    #[inline(always)]
    fn col(&self) -> &SelfRefCol<V, M, PinVec<V>> {
        self.col
    }
}

impl<'a, T, M> HasSinglyEnds<T, M> for ListSlice<'a, Singly<T>, M>
where
    M: MemoryPolicy<Singly<T>>,
{
    #[inline(always)]
    fn ends(&self) -> &<Singly<T> as Variant>::Ends {
        &self.ends
    }
}

impl<'a, T, M> HasDoublyEnds<T, M> for ListSlice<'a, Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
{
    #[inline(always)]
    fn ends(&self) -> &<Doubly<T> as Variant>::Ends {
        &self.ends
    }
}
