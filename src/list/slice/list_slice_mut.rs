use crate::{
    list::helper_traits::{
        HasCol, HasColMut, HasDoublyEnds, HasDoublyEndsMut, HasSinglyEnds, HasSinglyEndsMut,
    },
    type_aliases::{DefaultMemory, PinVec},
    variant::ListVariant,
    Doubly, List, Singly,
};
use orx_selfref_col::{MemoryPolicy, SelfRefCol, Variant};

/// A mutable slice of a linked list.
///
/// Note that a list slice itself behaves pretty much like a linked list.
/// However, it does not own the data, but provides a view on it, just as a slice of a vec.
pub struct ListSliceMut<'a, V, M = DefaultMemory<V>>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
{
    pub(crate) list: &'a mut List<V, M>,
    pub(crate) ends: V::Ends,
}

impl<'a, V, M> HasCol<V, M> for ListSliceMut<'a, V, M>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
{
    #[inline(always)]
    fn col(&self) -> &SelfRefCol<V, M, PinVec<V>> {
        &self.list.0
    }
}

impl<'a, V, M> HasColMut<V, M> for ListSliceMut<'a, V, M>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
{
    #[inline(always)]
    fn col_mut(&mut self) -> &mut SelfRefCol<V, M, PinVec<V>> {
        &mut self.list.0
    }
}

impl<'a, T, M> HasSinglyEnds<T, M> for ListSliceMut<'a, Singly<T>, M>
where
    M: MemoryPolicy<Singly<T>>,
{
    #[inline(always)]
    fn ends(&self) -> &<Singly<T> as Variant>::Ends {
        &self.ends
    }
}

impl<'a, T, M> HasSinglyEndsMut<T, M> for ListSliceMut<'a, Singly<T>, M>
where
    M: MemoryPolicy<Singly<T>>,
{
    #[inline(always)]
    fn ends_mut(&mut self) -> &mut <Singly<T> as Variant>::Ends {
        &mut self.ends
    }
}

impl<'a, T, M> HasDoublyEnds<T, M> for ListSliceMut<'a, Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
{
    #[inline(always)]
    fn ends(&self) -> &<Doubly<T> as Variant>::Ends {
        &self.ends
    }
}

impl<'a, T, M> HasDoublyEndsMut<T, M> for ListSliceMut<'a, Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
{
    #[inline(always)]
    fn ends_mut(&mut self) -> &mut <Doubly<T> as Variant>::Ends {
        &mut self.ends
    }
}
