use crate::{
    Doubly, List, Singly,
    list::helper_traits::{
        HasCol, HasColMut, HasDoublyEnds, HasDoublyEndsMut, HasSinglyEnds, HasSinglyEndsMut,
    },
    type_aliases::{DefaultMemory, DefaultPinVec},
    variant::ListVariant,
};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node, SelfRefCol, Variant};

/// A mutable slice of a linked list.
///
/// Note that a list slice itself behaves pretty much like a linked list.
/// However, it does not own the data, but provides a view on it, just as a slice of a vec.
pub struct ListSliceMut<'a, V, M = DefaultMemory<V>, P = DefaultPinVec<V>>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
    P: PinnedVec<Node<V>>,
{
    pub(crate) list: &'a mut List<V, M, P>,
    pub(crate) ends: V::Ends,
}

impl<V, M, P> HasCol<V, M, P> for ListSliceMut<'_, V, M, P>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
    P: PinnedVec<Node<V>>,
{
    #[inline(always)]
    fn col(&self) -> &SelfRefCol<V, M, P> {
        &self.list.0
    }
}

impl<V, M, P> HasColMut<V, M, P> for ListSliceMut<'_, V, M, P>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
    P: PinnedVec<Node<V>>,
{
    #[inline(always)]
    fn col_mut(&mut self) -> &mut SelfRefCol<V, M, P> {
        &mut self.list.0
    }
}

impl<T, M, P> HasSinglyEnds<T, M, P> for ListSliceMut<'_, Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
    #[inline(always)]
    fn ends(&self) -> &<Singly<T> as Variant>::Ends {
        &self.ends
    }
}

impl<T, M, P> HasSinglyEndsMut<T, M, P> for ListSliceMut<'_, Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
    #[inline(always)]
    fn ends_mut(&mut self) -> &mut <Singly<T> as Variant>::Ends {
        &mut self.ends
    }
}

impl<T, M, P> HasDoublyEnds<T, M, P> for ListSliceMut<'_, Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    #[inline(always)]
    fn ends(&self) -> &<Doubly<T> as Variant>::Ends {
        &self.ends
    }
}

impl<T, M, P> HasDoublyEndsMut<T, M, P> for ListSliceMut<'_, Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    #[inline(always)]
    fn ends_mut(&mut self) -> &mut <Doubly<T> as Variant>::Ends {
        &mut self.ends
    }
}
