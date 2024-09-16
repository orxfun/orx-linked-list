use crate::{
    type_aliases::{DefaultMemory, PinVec},
    variant::ListVariant,
    Doubly, Singly,
};
use helper_traits::{
    HasCol, HasColMut, HasDoublyEnds, HasDoublyEndsMut, HasSinglyEnds, HasSinglyEndsMut,
};
use orx_selfref_col::{MemoryPolicy, SelfRefCol};

pub(crate) mod ends_traits;
pub(crate) mod helper_traits;
pub(crate) mod iter_traits;

mod common_traits;
mod consuming;
mod get;
mod get_doubly;
mod idx_doubly;
mod idx_singly;
mod linear;
mod linear_eq;
mod mut_doubly;
mod mut_singly;
mod mutate;
mod new;
mod reclaim;
pub(crate) mod slice;

/// Core linked list structure which might represent either of the two variants
/// doubly or singly linked with different memory policies such as auto-reclaim or lazy-reclaim.
/// See [`DoublyList`], [`DoublyListLazy`], [`SinglyList`], [`SinglyListLazy`]
/// for variants.
///
/// [`DoublyList`]: crate::DoublyList
/// [`DoublyListLazy`]: crate::DoublyListLazy
/// [`SinglyList`]: crate::SinglyList
/// [`SinglyListLazy`]: crate::SinglyListLazy
pub struct List<V, M = DefaultMemory<V>>(pub(crate) SelfRefCol<V, M, PinVec<V>>)
where
    V: ListVariant,
    M: MemoryPolicy<V>;

// helper traits

impl<V, M> HasCol<V, M> for List<V, M>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
{
    #[inline(always)]
    fn col(&self) -> &SelfRefCol<V, M, PinVec<V>> {
        &self.0
    }
}

impl<V, M> HasColMut<V, M> for List<V, M>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
{
    #[inline(always)]
    fn col_mut(&mut self) -> &mut SelfRefCol<V, M, PinVec<V>> {
        &mut self.0
    }
}

impl<T, M> HasDoublyEnds<T, M> for List<Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
{
    fn ends(&self) -> &<Doubly<T> as orx_selfref_col::Variant>::Ends {
        self.0.ends()
    }
}

impl<T, M> HasDoublyEndsMut<T, M> for List<Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
{
    fn ends_mut(&mut self) -> &mut <Doubly<T> as orx_selfref_col::Variant>::Ends {
        self.0.ends_mut()
    }
}

impl<T, M> HasSinglyEnds<T, M> for List<Singly<T>, M>
where
    M: MemoryPolicy<Singly<T>>,
{
    fn ends(&self) -> &<Singly<T> as orx_selfref_col::Variant>::Ends {
        self.0.ends()
    }
}

impl<T, M> HasSinglyEndsMut<T, M> for List<Singly<T>, M>
where
    M: MemoryPolicy<Singly<T>>,
{
    fn ends_mut(&mut self) -> &mut <Singly<T> as orx_selfref_col::Variant>::Ends {
        self.0.ends_mut()
    }
}
