use crate::{
    type_aliases::OOB, variant::Doubly, DoublyEnds, DoublyEndsMut, DoublyIdx, List, ListSlice,
    ListSliceMut, Singly, SinglyEnds, SinglyEndsMut, SinglyIdx,
};
use core::ops::{Index, IndexMut};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node};

// doubly

impl<'i, T, M, P> Index<&'i DoublyIdx<T>> for List<Doubly<T>, M, P>
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
    fn index(&self, index: &'i DoublyIdx<T>) -> &Self::Output {
        self.get(index).expect(OOB)
    }
}

impl<'i, T, M, P> Index<&'i DoublyIdx<T>> for ListSlice<'_, Doubly<T>, M, P>
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
    fn index(&self, index: &'i DoublyIdx<T>) -> &Self::Output {
        self.get(index).expect(OOB)
    }
}

impl<'i, T, M, P> Index<&'i DoublyIdx<T>> for ListSliceMut<'_, Doubly<T>, M, P>
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
    fn index(&self, index: &'i DoublyIdx<T>) -> &Self::Output {
        self.get(index).expect(OOB)
    }
}

impl<'i, T, M, P> IndexMut<&'i DoublyIdx<T>> for List<Doubly<T>, M, P>
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
    fn index_mut(&mut self, index: &'i DoublyIdx<T>) -> &mut Self::Output {
        self.get_mut(index).expect(OOB)
    }
}

impl<'i, T, M, P> IndexMut<&'i DoublyIdx<T>> for ListSliceMut<'_, Doubly<T>, M, P>
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
    fn index_mut(&mut self, index: &'i DoublyIdx<T>) -> &mut Self::Output {
        self.get_mut(index).expect(OOB)
    }
}

// singly

impl<'i, T, M, P> Index<&'i SinglyIdx<T>> for List<Singly<T>, M, P>
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
    fn index(&self, index: &'i SinglyIdx<T>) -> &Self::Output {
        self.get(index).expect(OOB)
    }
}

impl<'i, T, M, P> Index<&'i SinglyIdx<T>> for ListSlice<'_, Singly<T>, M, P>
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
    fn index(&self, index: &'i SinglyIdx<T>) -> &Self::Output {
        self.get(index).expect(OOB)
    }
}

impl<'i, T, M, P> Index<&'i SinglyIdx<T>> for ListSliceMut<'_, Singly<T>, M, P>
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
    fn index(&self, index: &'i SinglyIdx<T>) -> &Self::Output {
        self.get(index).expect(OOB)
    }
}

impl<'i, T, M, P> IndexMut<&'i SinglyIdx<T>> for List<Singly<T>, M, P>
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
    fn index_mut(&mut self, index: &'i SinglyIdx<T>) -> &mut Self::Output {
        self.get_mut(index).expect(OOB)
    }
}

impl<'i, T, M, P> IndexMut<&'i SinglyIdx<T>> for ListSliceMut<'_, Singly<T>, M, P>
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
    fn index_mut(&mut self, index: &'i SinglyIdx<T>) -> &mut Self::Output {
        self.get_mut(index).expect(OOB)
    }
}
