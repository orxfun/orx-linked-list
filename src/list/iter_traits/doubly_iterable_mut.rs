use crate::{
    iter::{DoublyIterMut, DoublyIterMutChain},
    list::helper_traits::HasDoublyEndsMut,
    type_aliases::{BACK_IDX, FRONT_IDX, OOB},
    Doubly, DoublyIdx,
};
use core::iter::Rev;
use orx_selfref_col::MemoryPolicy;

/// Iterator methods for doubly linked lists.
pub trait DoublyIterableMut<T, M>: HasDoublyEndsMut<T, M>
where
    M: MemoryPolicy<Doubly<T>>,
    Self: Sized,
{
    /// Returns a double-ended iterator of mutable references to elements of the list from front to back.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
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
    fn iter_mut<'a>(&'a mut self) -> DoublyIterMut<T>
    where
        M: 'a,
    {
        let a = self.ends().get(FRONT_IDX);
        let b = self.ends().get(BACK_IDX);
        DoublyIterMut::new(self.col_mut(), a, b)
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
    /// let mut list = DoublyList::new();
    ///
    /// list.push_back(0);
    /// list.push_back(1);
    /// let idx = list.push_back(2);
    /// list.push_back(3);
    ///
    /// assert!(list.eq_to_iter_vals([0, 1, 2, 3]));
    ///
    /// for x in list.iter_mut_from(&idx) {
    ///     *x += 10;
    /// }
    ///
    /// assert!(list.eq_to_iter_vals([0, 1, 12, 13]));
    /// ```
    fn iter_mut_from<'a>(&'a mut self, idx: &DoublyIdx<T>) -> DoublyIterMut<T>
    where
        M: 'a,
    {
        let a = self.col().try_get_ptr(idx).expect(OOB);
        let b = self.ends().get(BACK_IDX);
        DoublyIterMut::new(self.col_mut(), Some(a), b)
    }

    /// Creates a mutable backward iterator:
    /// * from the node with the given `idx`
    /// * to the `front` of the list.
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
    /// let mut list = DoublyList::new();
    ///
    /// list.push_back(0);
    /// list.push_back(1);
    /// let idx = list.push_back(2);
    /// list.push_back(3);
    ///
    /// assert!(list.eq_to_iter_vals([0, 1, 2, 3]));
    ///
    /// for x in list.iter_mut_backward_from(&idx) {
    ///     *x += 10;
    /// }
    ///
    /// assert!(list.eq_to_iter_vals([10, 11, 12, 3]));
    /// ```
    fn iter_mut_backward_from<'a>(&'a mut self, idx: &DoublyIdx<T>) -> Rev<DoublyIterMut<T>>
    where
        M: 'a,
    {
        let b = self.col().try_get_ptr(idx).expect(OOB);
        let a = self.col().ends().get(FRONT_IDX);
        DoublyIterMut::new(self.col_mut(), a, Some(b)).rev()
    }

    /// Creates a mutable forward iterator starting from the `pivot_idx` and ending at the element before it.
    ///
    /// The iterator jumps to front when it hits the back; and hence,
    /// gives the linked list a circular behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// // a simple scan impl
    /// fn scan<'a, I: Iterator<Item = &'a mut i32>>(mut values: I) {
    ///     if let Some(first) = values.next() {
    ///         let mut acc = *first;
    ///         while let Some(x) = values.next() {
    ///             let new_acc = acc + *x;
    ///             *x += acc;
    ///             acc = new_acc;
    ///         }
    ///     }
    /// }
    ///
    /// // regular scan
    /// let mut list: DoublyList<_> = (0..5).collect();
    /// assert!(list.eq_to_iter_vals([0, 1, 2, 3, 4]));
    ///
    /// scan(list.iter_mut());
    /// assert!(list.eq_to_iter_vals([0, 1, 3, 6, 10]));
    ///
    /// // circular scan starting from any pivot point
    /// let mut list: DoublyList<_> = (0..5).collect();
    /// let idx: Vec<_> = list.indices().collect();
    ///
    /// scan(list.ring_iter_mut(&idx[3]));
    /// assert!(list.eq_to_iter_vals([7, 8, 10, 3, 7]));
    /// ```
    fn ring_iter_mut<'a>(&'a mut self, pivot_idx: &DoublyIdx<T>) -> DoublyIterMutChain<'a, T>
    where
        M: 'a,
    {
        let a1 = self.col().try_get_ptr(pivot_idx).expect(OOB);
        let b1 = self.ends().get(BACK_IDX).expect(OOB);

        let a2 = self.ends().get(FRONT_IDX).expect(OOB);
        let b2 = self.col().node(&a1).prev().get();

        let second = match a1 == a2 {
            true => [None, None],
            false => [Some(a2), b2],
        };
        let first = [Some(a1), Some(b1)];

        DoublyIterMutChain::new(self.col_mut(), first, second)
    }
}

impl<L, T, M> DoublyIterableMut<T, M> for L
where
    L: HasDoublyEndsMut<T, M>,
    M: MemoryPolicy<Doubly<T>>,
{
}
