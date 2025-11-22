use super::{List, helper_traits::HasDoublyEnds, slice::ListSlice};
use crate::{DoublyIdx, variant::Doubly};
use core::ops::RangeBounds;
use orx_selfref_col::MemoryPolicy;

impl<T, M> List<Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
{
    /// Creates and returns a slice of the list between the given `range` of indices.
    ///
    /// Note that a linked list slice itself also behaves like a linked list,
    /// reflecting the recursive nature of the data type.
    /// However, it does not own the data.
    /// It is rather a view, like a slice is a view to a vec.
    ///
    /// Note that slicing might be useful in various ways.
    /// For instance, we can keep indices of several critical elements of the list.
    /// We can then get all elements before, after or between any pair of these indices.
    /// Or we can combine the list with an indices vector, which provides the linked list
    /// a vec-like usage
    /// * with the disadvantage of using more memory, and
    /// * with the advantage of constant time insertions, removals or moves.
    ///
    /// # Panics
    ///
    /// Panics if any of indices of the range bounds is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// list.push_back(3);
    /// list.push_front(1);
    /// list.push_front(7);
    /// list.push_back(4);
    /// list.push_front(9);
    ///
    /// let expected_values = vec![9, 7, 1, 3, 4];
    ///
    /// assert!(list.eq_to_iter_refs(&expected_values));
    /// assert!(list.slice(..).eq_to_iter_refs(&expected_values));
    ///
    /// let idx: Vec<_> = list.indices().collect();
    ///
    /// let slice = list.slice(&idx[1]..=&idx[3]);
    /// assert_eq!(slice.front(), Some(&7));
    /// assert_eq!(slice.back(), Some(&3));
    /// assert!(slice.eq_to_iter_vals([7, 1, 3]));
    ///
    /// let sum: usize = slice.iter().sum();
    /// assert_eq!(sum, 11);
    /// ```
    ///
    /// Note that the linked list and its slices are directed.
    /// In other words, it does not by default have a cyclic behavior.
    /// Therefore, if the end of the `range` is before the beginning,
    /// the slice will stop at the `back` of the list.
    /// See the following example for clarification.
    ///
    /// Currently, cyclic or ring behavior can be achieved by `ring_iter` method.
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let list: DoublyList<_> = (0..10).collect();
    /// let idx: Vec<_> = list.indices().collect();
    ///
    /// // a..b where b comes later, hence, we get the slice a..b
    /// let slice = list.slice(&idx[1]..&idx[4]);
    /// assert!(slice.eq_to_iter_vals([1, 2, 3]));
    ///
    /// // a..b where b comes earlier, then, we get the slice a..back
    /// let slice = list.slice(&idx[4]..&idx[1]);
    /// assert!(slice.eq_to_iter_vals([4, 5, 6, 7, 8, 9]));
    /// ```
    pub fn slice<R>(&self, range: R) -> ListSlice<'_, Doubly<T>, M>
    where
        R: RangeBounds<DoublyIdx<T>>,
    {
        let ends = self.slice_ends(range).expect("invalid indices in range");
        ListSlice { col: &self.0, ends }
    }
}
