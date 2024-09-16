use crate::{
    iter::{DoublyIter, DoublyIterPtr, DoublyLinkIter},
    list::helper_traits::HasDoublyEnds,
    type_aliases::{BACK_IDX, FRONT_IDX, OOB},
    Doubly, DoublyIdx,
};
use core::iter::{Chain, Rev};
use orx_selfref_col::MemoryPolicy;

/// Iterator methods for doubly linked lists.
pub trait DoublyIterable<T, M>: HasDoublyEnds<T, M>
where
    M: MemoryPolicy<Doubly<T>>,
    Self: Sized,
{
    /// Returns a double-ended iterator of pointers to the elements of the list from front to back.
    fn iter_ptr<'a>(&'a self) -> DoublyIterPtr<T>
    where
        M: 'a,
    {
        let a = self.ends().get(FRONT_IDX);
        let b = self.ends().get(BACK_IDX);
        DoublyIterPtr::new(self.col(), a, b)
    }

    /// Returns a double-ended iterator to elements of the list:
    /// * `next` iterates from front-to-back, while
    /// * `next_back` iterates from back-to-front.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// // a -> b -> c -> d
    /// list.push_front('b');
    /// list.push_back('c');
    /// list.push_back('d');
    /// list.push_front('a');
    ///
    /// // forward iteration
    /// assert_eq!(
    ///     list.iter().copied().collect::<Vec<_>>(),
    ///     &['a', 'b', 'c', 'd']
    /// );
    ///
    /// // backward iteration
    /// assert_eq!(
    ///     list.iter().rev().copied().collect::<Vec<_>>(),
    ///     &['d', 'c', 'b', 'a']
    /// );
    ///
    /// // mixed iteration
    /// let mut iter = list.iter();
    ///
    /// assert_eq!(Some(&'a'), iter.next());
    /// assert_eq!(Some(&'d'), iter.next_back());
    /// assert_eq!(Some(&'b'), iter.next());
    /// assert_eq!(Some(&'c'), iter.next());
    /// assert!(iter.next().is_none());
    /// ```
    fn iter<'a>(&'a self) -> DoublyIter<T>
    where
        M: 'a,
    {
        let a = self.ends().get(FRONT_IDX);
        let b = self.ends().get(BACK_IDX);
        DoublyIter::new(self.col(), a, b)
    }

    /// Creates a forward iterator that yields pairs of successive elements representing links.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let tour: DoublyList<_> = ['a', 'b', 'c', 'd', 'e'].into_iter().collect();
    ///
    /// let mut iter = tour.iter_links();
    ///
    /// assert_eq!(iter.next(), Some((&'a', &'b')));
    /// assert_eq!(iter.next(), Some((&'b', &'c')));
    /// assert_eq!(iter.next(), Some((&'c', &'d')));
    /// assert_eq!(iter.next(), Some((&'d', &'e')));
    ///
    /// assert_eq!(iter.next(), None);
    /// ```
    fn iter_links<'a>(&'a self) -> DoublyLinkIter<'a, T>
    where
        M: 'a,
    {
        let a = self.ends().get(FRONT_IDX);
        let b = a.as_ref().and_then(|a| self.col().node(a).next().get());
        let begin = match (a, b) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        };
        let end = self.ends().get(BACK_IDX);
        DoublyLinkIter::new(self.col(), begin, end)
    }

    /// Returns an iterator of indices of elements of the list.
    ///
    /// Recall that indices are used to enable constant time access to any place of the list.
    /// Methods adding an element to the list such as `push_front` or `insert` return the corresponding index of the element.
    ///
    /// This method, on the other hand, is useful to collect all indices at once, probably after a reorganization.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// list.push_back(1);
    /// list.push_front(0);
    /// list.push_back(2);
    /// assert!(list.eq_to_iter_vals([0, 1, 2]));
    ///
    /// let idx: Vec<_> = list.indices().collect();
    ///
    /// assert_eq!(list.get(&idx[1]), Some(&1));
    ///
    /// // O(1) mutations through indices
    /// list.insert_next_to(&idx[0], 42);
    /// list.insert_prev_to(&idx[2], 7);
    /// list.remove(&idx[1]);
    ///
    /// assert!(list.eq_to_iter_vals([0, 42, 7, 2]));
    /// ```
    fn indices<'a>(&'a self) -> impl Iterator<Item = DoublyIdx<T>>
    where
        M: 'a,
        T: 'a,
    {
        let s = self.col().memory_state();
        self.iter_ptr().map(move |ptr| DoublyIdx::new(s, &ptr))
    }

    // idx

    /// Creates a forward iterator starting from the `pivot_idx` and ending at the element before it.
    ///
    /// The iterator jumps to front when it hits the back; and hence,
    /// gives the linked list a circular behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let list: DoublyList<_> = (0..8).collect();
    /// let idx: Vec<_> = list.indices().collect();
    ///
    /// let iter = list.ring_iter(&idx[2]);
    /// assert_eq!(iter.copied().collect::<Vec<_>>(), [2, 3, 4, 5, 6, 7, 0, 1]);
    ///
    /// let iter = list.ring_iter(&idx[4]);
    /// assert_eq!(iter.copied().collect::<Vec<_>>(), [4, 5, 6, 7, 0, 1, 2, 3]);
    ///
    /// // ring iterator is also double-ended
    /// let iter = list.ring_iter(&idx[4]).rev();
    /// assert_eq!(iter.copied().collect::<Vec<_>>(), [3, 2, 1, 0, 7, 6, 5, 4]);
    ///
    /// // ring iterators are also available for slices
    /// let slice = list.slice(&idx[3]..&idx[7]);
    /// assert!(slice.eq_to_iter_vals([3, 4, 5, 6]));
    ///
    /// let iter = slice.ring_iter(&idx[4]);
    /// assert_eq!(iter.copied().collect::<Vec<_>>(), [4, 5, 6, 3,]);
    ///
    /// let iter = slice.ring_iter(&idx[6]);
    /// assert_eq!(iter.copied().collect::<Vec<_>>(), [6, 3, 4, 5]);
    /// ```
    fn ring_iter<'a>(
        &'a self,
        pivot_idx: &DoublyIdx<T>,
    ) -> Chain<DoublyIter<'a, T>, DoublyIter<'a, T>>
    where
        M: 'a,
    {
        let iter1 = self.iter_from(pivot_idx);

        let pivot = self.col().try_get_ptr(pivot_idx).expect(OOB);
        let a = self.ends().get(FRONT_IDX).expect(OOB);

        let iter2 = match pivot == a {
            true => DoublyIter::new(self.col(), None, None),
            false => match self.col().node(&pivot).prev().get() {
                Some(b) => DoublyIter::new(self.col(), Some(a), Some(b)),
                None => DoublyIter::new(self.col(), None, None),
            },
        };

        iter1.chain(iter2)
    }

    /// Creates a forward iterator:
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
    /// let mut iter = list.iter_from(&idx);
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn iter_from<'a>(&'a self, idx: &DoublyIdx<T>) -> DoublyIter<T>
    where
        M: 'a,
    {
        let a = self.col().try_get_ptr(idx).expect(OOB);
        let b = self.ends().get(BACK_IDX);
        DoublyIter::new(self.col(), Some(a), b)
    }

    /// Creates a backward iterator:
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
    /// let mut iter = list.iter_backward_from(&idx);
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn iter_backward_from<'a>(&'a self, idx: &DoublyIdx<T>) -> Rev<DoublyIter<T>>
    where
        M: 'a,
    {
        let b = self.col().try_get_ptr(idx).expect(OOB);
        let a = self.ends().get(BACK_IDX);
        DoublyIter::new(self.col(), a, Some(b)).rev()
    }

    /// Creates a forward iterator that yields pairs of successive elements representing links:
    /// * starting from the node with the given `idx`
    /// * to the `back` of the list.
    ///
    /// # Panics
    ///
    /// Panics if the index is invalid; i.e., `idx_err` does not return None.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let tour: DoublyList<_> = ['a', 'b', 'c', 'd', 'e'].into_iter().collect();
    /// let idx: Vec<_> = tour.indices().collect();
    ///
    /// let mut iter = tour.iter_links_from(&idx[1]);
    ///
    /// assert_eq!(iter.next(), Some((&'b', &'c')));
    /// assert_eq!(iter.next(), Some((&'c', &'d')));
    /// assert_eq!(iter.next(), Some((&'d', &'e')));
    ///
    /// assert_eq!(iter.next(), None);
    /// ```
    fn iter_links_from<'a>(&'a self, idx: &DoublyIdx<T>) -> DoublyLinkIter<'a, T>
    where
        M: 'a,
    {
        let a = self.col().try_get_ptr(idx).expect(OOB);
        let b = self.col().node(&a).next().get();
        let begin = b.map(|b| (a, b));
        let end = self.ends().get(BACK_IDX);
        DoublyLinkIter::new(self.col(), begin, end)
    }

    // debug

    /// Returns true if the elements of the iterator are equal to those of the list.
    fn eq_to_iter_vals<I>(&self, iter: I) -> bool
    where
        I: IntoIterator<Item = T>,
        T: PartialEq,
    {
        let mut iter = iter.into_iter();

        for x in self.iter() {
            match iter.next() {
                Some(y) if x != &y => return false,
                None => return false,
                _ => {}
            }
        }

        iter.next().is_none()
    }

    /// Returns true if the elements of the iterator are equal to those of the list.
    fn eq_to_iter_refs<'a, I>(&self, iter: I) -> bool
    where
        I: IntoIterator<Item = &'a T>,
        T: 'a + PartialEq,
    {
        let mut iter = iter.into_iter();

        for x in self.iter() {
            match iter.next() {
                Some(y) if x != y => return false,
                None => return false,
                _ => {}
            }
        }

        iter.next().is_none()
    }

    /// Returns a string representing the values in the underlying storage of the list.
    #[cfg(test)]
    #[allow(clippy::format_collect)]
    fn debug_nodes(&self) -> alloc::string::String
    where
        T: std::fmt::Display,
    {
        use alloc::string::ToString;
        use orx_pinned_vec::PinnedVec;
        self.col()
            .nodes()
            .iter()
            .map(|n| match n.data() {
                Some(x) => alloc::format!("{} ", x),
                None => "x ".to_string(),
            })
            .collect()
    }

    /// Returns a space separated text of elements from front to back.
    #[cfg(test)]
    #[allow(clippy::format_collect)]
    fn debug_values(&self) -> alloc::string::String
    where
        T: std::fmt::Display,
    {
        self.iter().map(|x| alloc::format!("{} ", x)).collect()
    }

    /// Returns a space separated text of prev-elements of nodes from front to back.
    #[cfg(test)]
    #[allow(clippy::format_collect, clippy::unwrap_used)]
    fn debug_prev(&self) -> alloc::string::String
    where
        T: std::fmt::Display,
    {
        use alloc::string::ToString;
        use orx_pinned_vec::PinnedVec;

        self.col()
            .nodes()
            .iter()
            .map(|n| match n.prev().get() {
                Some(x) => {
                    let x = self.col().node(&x).data().unwrap();
                    alloc::format!("{} ", x)
                }
                None => "x ".to_string(),
            })
            .collect()
    }

    /// Returns a space separated text of next-elements of nodes from front to back.
    #[cfg(test)]
    #[allow(clippy::format_collect, clippy::unwrap_used)]
    fn debug_next(&self) -> alloc::string::String
    where
        T: std::fmt::Display,
    {
        use alloc::string::ToString;
        use orx_pinned_vec::PinnedVec;

        self.col()
            .nodes()
            .iter()
            .map(|n| match n.next().get() {
                Some(x) => {
                    let x = self.col().node(&x).data().unwrap();
                    alloc::format!("{} ", x)
                }
                None => "x ".to_string(),
            })
            .collect()
    }
}

impl<L, T, M> DoublyIterable<T, M> for L
where
    L: HasDoublyEnds<T, M>,
    M: MemoryPolicy<Doubly<T>>,
{
}
