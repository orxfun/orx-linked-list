use crate::{
    iter::SinglyIterMut, list::helper_traits::HasSinglyEndsMut, type_aliases::OOB, Singly,
    SinglyIdx,
};
use orx_selfref_col::MemoryPolicy;

/// Iterator methods for Singly linked lists.
pub trait SinglyIterableMut<T, M>: HasSinglyEndsMut<T, M>
where
    M: MemoryPolicy<Singly<T>>,
    Self: Sized,
{
    /// Returns a double-ended iterator of mutable references to elements of the list from front to back.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = SinglyList::new();
    ///
    /// list.push_front(2);
    /// list.push_front(1);
    /// list.push_front(0);
    /// assert!(list.eq_to_iter_vals([0, 1, 2]));
    ///
    /// for x in list.iter_mut() {
    ///     *x += 40;
    /// }
    ///
    /// assert!(list.eq_to_iter_vals([40, 41, 42]));
    /// ```
    fn iter_mut<'a>(&'a mut self) -> SinglyIterMut<T>
    where
        M: 'a,
    {
        let a = self.ends().get();
        SinglyIterMut::new(self.col_mut(), a)
    }

    // idx

    /// Creates a mutable forward iterator:
    /// * from the node with the given `idx`
    /// * to the `back` of the list.
    ///
    /// # Panics
    ///
    /// Panics if the index is invalid; i.e., `idx_err` does not return None.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = SinglyList::new();
    ///
    /// list.push_front(3);
    /// list.push_front(2);
    /// let idx = list.push_front(1);
    /// list.push_front(0);
    ///
    /// assert!(list.eq_to_iter_vals([0, 1, 2, 3]));
    ///
    /// for x in list.iter_mut_from(&idx) {
    ///     *x += 10;
    /// }
    ///
    /// assert!(list.eq_to_iter_vals([0, 11, 12, 13]));
    /// ```
    fn iter_mut_from<'a>(&'a mut self, idx: &SinglyIdx<T>) -> SinglyIterMut<T>
    where
        M: 'a,
    {
        let a = self.col().try_get_ptr(idx).expect(OOB);
        SinglyIterMut::new(self.col_mut(), Some(a))
    }
}

impl<L, T, M> SinglyIterableMut<T, M> for L
where
    L: HasSinglyEndsMut<T, M>,
    M: MemoryPolicy<Singly<T>>,
{
}
