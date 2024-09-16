use super::List;
use crate::variant::ListVariant;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::MemoryPolicy;

impl<V, M> List<V, M>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
{
    /// Clears the list removing all elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// list.push_front('b');
    /// list.push_back('c');
    /// list.push_front('a');
    ///
    /// assert_eq!(3, list.len());
    ///
    /// list.clear();
    /// assert!(list.is_empty());
    /// assert!(list.front().is_none());
    /// ```
    #[inline(always)]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns an arbitrary order iterator of mutable references to elements of the list from front to back.
    ///
    /// Note that the iterator created by `iter_mut_x` is often faster than that created by `iter_mut`;
    /// and hence, can be preferred whenever the iteration order does not matter.
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
    /// for x in list.iter_mut_x() {
    ///     *x += 40;
    /// }
    ///
    /// assert!(list.eq_to_iter_vals([40, 41, 42]));
    /// ```
    pub fn iter_mut_x(&mut self) -> impl Iterator<Item = &mut V::Item> {
        self.0.nodes_mut().iter_mut().filter_map(|x| x.data_mut())
    }
}
