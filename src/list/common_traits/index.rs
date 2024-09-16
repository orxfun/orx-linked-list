use crate::{
    type_aliases::OOB, variant::Doubly, DoublyEnds, DoublyEndsMut, DoublyIdx, List, ListSlice,
    ListSliceMut, Singly, SinglyEnds, SinglyEndsMut, SinglyIdx,
};
use orx_selfref_col::MemoryPolicy;
use std::ops::{Index, IndexMut};

// doubly

impl<'i, T, M> Index<&'i DoublyIdx<T>> for List<Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
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

impl<'a, 'i, T, M> Index<&'i DoublyIdx<T>> for ListSlice<'a, Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
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

impl<'a, 'i, T, M> Index<&'i DoublyIdx<T>> for ListSliceMut<'a, Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
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

impl<'i, T, M> IndexMut<&'i DoublyIdx<T>> for List<Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
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

impl<'a, 'i, T, M> IndexMut<&'i DoublyIdx<T>> for ListSliceMut<'a, Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
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

impl<'i, T, M> Index<&'i SinglyIdx<T>> for List<Singly<T>, M>
where
    M: MemoryPolicy<Singly<T>>,
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

impl<'a, 'i, T, M> Index<&'i SinglyIdx<T>> for ListSlice<'a, Singly<T>, M>
where
    M: MemoryPolicy<Singly<T>>,
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

impl<'a, 'i, T, M> Index<&'i SinglyIdx<T>> for ListSliceMut<'a, Singly<T>, M>
where
    M: MemoryPolicy<Singly<T>>,
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

impl<'i, T, M> IndexMut<&'i SinglyIdx<T>> for List<Singly<T>, M>
where
    M: MemoryPolicy<Singly<T>>,
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

impl<'a, 'i, T, M> IndexMut<&'i SinglyIdx<T>> for ListSliceMut<'a, Singly<T>, M>
where
    M: MemoryPolicy<Singly<T>>,
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
