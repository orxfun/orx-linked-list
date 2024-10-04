use crate::{
    type_aliases::{DefaultMemory, DefaultPinVec},
    variant::ListVariant,
    Doubly, Singly,
};
use helper_traits::{
    HasCol, HasColMut, HasDoublyEnds, HasDoublyEndsMut, HasSinglyEnds, HasSinglyEndsMut,
};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node, SelfRefCol};

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
mod mut_doubly_recursive;
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
pub struct List<V, M = DefaultMemory<V>, P = DefaultPinVec<V>>(pub(crate) SelfRefCol<V, M, P>)
where
    V: ListVariant,
    M: MemoryPolicy<V>,
    P: PinnedVec<Node<V>>;

// helper traits

impl<V, M, P> HasCol<V, M, P> for List<V, M, P>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
    P: PinnedVec<Node<V>>,
{
    #[inline(always)]
    fn col(&self) -> &SelfRefCol<V, M, P> {
        &self.0
    }
}

impl<V, M, P> HasColMut<V, M, P> for List<V, M, P>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
    P: PinnedVec<Node<V>>,
{
    #[inline(always)]
    fn col_mut(&mut self) -> &mut SelfRefCol<V, M, P> {
        &mut self.0
    }
}

impl<T, M, P> HasDoublyEnds<T, M, P> for List<Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    fn ends(&self) -> &<Doubly<T> as orx_selfref_col::Variant>::Ends {
        self.0.ends()
    }
}

impl<T, M, P> HasDoublyEndsMut<T, M, P> for List<Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    fn ends_mut(&mut self) -> &mut <Doubly<T> as orx_selfref_col::Variant>::Ends {
        self.0.ends_mut()
    }
}

impl<T, M, P> HasSinglyEnds<T, M, P> for List<Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
    fn ends(&self) -> &<Singly<T> as orx_selfref_col::Variant>::Ends {
        self.0.ends()
    }
}

impl<T, M, P> HasSinglyEndsMut<T, M, P> for List<Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
    fn ends_mut(&mut self) -> &mut <Singly<T> as orx_selfref_col::Variant>::Ends {
        self.0.ends_mut()
    }
}
