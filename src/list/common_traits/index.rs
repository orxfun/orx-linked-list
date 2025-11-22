use crate::{
    DoublyEnds, DoublyEndsMut, DoublyIdx, List, ListSlice, ListSliceMut, Singly, SinglyEnds,
    SinglyEndsMut, SinglyIdx, type_aliases::OOB, variant::Doubly,
};
use core::ops::{Index, IndexMut};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node};

// doubly

impl<T, M, P> Index<DoublyIdx<T>> for List<Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    type Output = T;

    /// Returns the element at the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if the `index` is invalid; i.e.,
    /// * `list.is_valid(index)` returns false, equivalently,
    /// * `list.idx_err(index)` returns the detail of the error.
    fn index(&self, index: DoublyIdx<T>) -> &Self::Output {
        self.get(index).expect(OOB)
    }
}

impl<T, M, P> Index<DoublyIdx<T>> for ListSlice<'_, Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    type Output = T;

    /// Returns the element at the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if the `index` is invalid; i.e.,
    /// * `list.is_valid(index)` returns false, equivalently,
    /// * `list.idx_err(index)` returns the detail of the error.
    fn index(&self, index: DoublyIdx<T>) -> &Self::Output {
        self.get(index).expect(OOB)
    }
}

impl<T, M, P> Index<DoublyIdx<T>> for ListSliceMut<'_, Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    type Output = T;

    /// Returns the element at the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if the `index` is invalid; i.e.,
    /// * `list.is_valid(index)` returns false, equivalently,
    /// * `list.idx_err(index)` returns the detail of the error.
    fn index(&self, index: DoublyIdx<T>) -> &Self::Output {
        self.get(index).expect(OOB)
    }
}

impl<'i, T, M, P> IndexMut<DoublyIdx<T>> for List<Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    /// Returns a mutable reference to the element at the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if the `index` is invalid; i.e.,
    /// * `list.is_valid(index)` returns false, equivalently,
    /// * `list.idx_err(index)` returns the detail of the error.
    fn index_mut(&mut self, index: DoublyIdx<T>) -> &mut Self::Output {
        self.get_mut(index).expect(OOB)
    }
}

impl<T, M, P> IndexMut<DoublyIdx<T>> for ListSliceMut<'_, Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    /// Returns a mutable reference to the element at the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if the `index` is invalid; i.e.,
    /// * `list.is_valid(index)` returns false, equivalently,
    /// * `list.idx_err(index)` returns the detail of the error.
    fn index_mut(&mut self, index: DoublyIdx<T>) -> &mut Self::Output {
        self.get_mut(index).expect(OOB)
    }
}

// singly

impl<T, M, P> Index<SinglyIdx<T>> for List<Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
    type Output = T;

    /// Returns the element at the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if the `index` is invalid; i.e.,
    /// * `list.is_valid(index)` returns false, equivalently,
    /// * `list.idx_err(index)` returns the detail of the error.
    fn index(&self, index: SinglyIdx<T>) -> &Self::Output {
        self.get(index).expect(OOB)
    }
}

impl<T, M, P> Index<SinglyIdx<T>> for ListSlice<'_, Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
    type Output = T;

    /// Returns the element at the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if the `index` is invalid; i.e.,
    /// * `list.is_valid(index)` returns false, equivalently,
    /// * `list.idx_err(index)` returns the detail of the error.
    fn index(&self, index: SinglyIdx<T>) -> &Self::Output {
        self.get(index).expect(OOB)
    }
}

impl<T, M, P> Index<SinglyIdx<T>> for ListSliceMut<'_, Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
    type Output = T;

    /// Returns the element at the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if the `index` is invalid; i.e.,
    /// * `list.is_valid(index)` returns false, equivalently,
    /// * `list.idx_err(index)` returns the detail of the error.
    fn index(&self, index: SinglyIdx<T>) -> &Self::Output {
        self.get(index).expect(OOB)
    }
}

impl<T, M, P> IndexMut<SinglyIdx<T>> for List<Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
    /// Returns a mutable reference to the element at the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if the `index` is invalid; i.e.,
    /// * `list.is_valid(index)` returns false, equivalently,
    /// * `list.idx_err(index)` returns the detail of the error.
    fn index_mut(&mut self, index: SinglyIdx<T>) -> &mut Self::Output {
        self.get_mut(index).expect(OOB)
    }
}

impl<T, M, P> IndexMut<SinglyIdx<T>> for ListSliceMut<'_, Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
    /// Returns a mutable reference to the element at the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if the `index` is invalid; i.e.,
    /// * `list.is_valid(index)` returns false, equivalently,
    /// * `list.idx_err(index)` returns the detail of the error.
    fn index_mut(&mut self, index: SinglyIdx<T>) -> &mut Self::Output {
        self.get_mut(index).expect(OOB)
    }
}
