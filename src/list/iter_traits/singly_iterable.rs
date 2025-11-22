use crate::{
    Singly, SinglyIdx,
    iter::{SinglyIter, SinglyIterPtr},
    list::helper_traits::HasSinglyEnds,
    pointers::SinglyPtr,
    type_aliases::OOB,
};
use orx_selfref_col::{MemoryPolicy, Node};
use orx_split_vec::PinnedVec;

/// Iterator methods for Singly linked lists.
pub trait SinglyIterable<T, M, P>: HasSinglyEnds<T, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
    Self: Sized,
{
    /// Returns a double-ended iterator of pointers to the elements of the list from front to back.
    fn iter_ptr<'a>(&'a self) -> SinglyIterPtr<'a, T, P>
    where
        M: 'a,
    {
        let a = self.ends().get().cloned();
        SinglyIterPtr::new(self.col(), a)
    }

    /// Returns a forward iterator to elements of the list from front to back.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = SinglyList::new();
    ///
    /// // a -> b -> c
    /// list.push_front('c');
    /// list.push_front('b');
    /// list.push_front('a');
    ///
    /// let mut iter = list.iter();
    ///
    /// assert_eq!(Some(&'a'), iter.next());
    /// assert_eq!(Some(&'b'), iter.next());
    /// assert_eq!(Some(&'c'), iter.next());
    /// assert!(iter.next().is_none());
    /// ```
    fn iter<'a>(&'a self) -> SinglyIter<'a, T, P>
    where
        M: 'a,
    {
        let a = self.ends().get().cloned();
        SinglyIter::new(self.col(), a)
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
    /// let mut list = SinglyList::new();
    ///
    /// list.push_front(2);
    /// list.push_front(1);
    /// list.push_front(0);
    /// assert!(list.eq_to_iter_vals([0, 1, 2]));
    ///
    /// let idx: Vec<_> = list.indices().collect();
    ///
    /// assert_eq!(list.get(&idx[1]), Some(&1));
    /// ```
    fn indices<'a>(&'a self) -> impl Iterator<Item = SinglyIdx<T>>
    where
        M: 'a,
        T: 'a,
        P: 'a,
    {
        let s = self.col().memory_state();
        self.iter_ptr().map(move |ptr| SinglyIdx::new(s, &ptr))
    }

    /// Returns an iterator of pointers to the elements of the list.
    ///
    /// Similar to indices, pointers are used to enable constant time access to any place of the list.
    /// They are thinner; however, have only some of the safety guarantees that indices have.
    fn pointers<'a>(&'a self) -> impl Iterator<Item = SinglyPtr<T>>
    where
        M: 'a,
        T: 'a,
        P: 'a,
    {
        self.iter_ptr()
    }

    // idx

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
    /// let mut list = SinglyList::new();
    ///
    /// list.push_front(3);
    /// list.push_front(2);
    /// let idx = list.push_front(1);
    /// list.push_front(0); // 0->1->2->3
    ///
    /// let mut iter = list.iter_from(&idx);
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn iter_from<'a>(&'a self, idx: SinglyIdx<T>) -> SinglyIter<'a, T, P>
    where
        M: 'a,
    {
        let a = self.col().try_get_ptr(idx).expect(OOB);
        SinglyIter::new(self.col(), Some(a))
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

    /// Returns a space separated text of next-elements of nodes from front to back.
    #[cfg(test)]
    #[allow(clippy::format_collect, clippy::unwrap_used)]
    fn debug_next(&self) -> alloc::string::String
    where
        T: std::fmt::Display,
    {
        use alloc::string::ToString;

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

impl<L, T, M, P> SinglyIterable<T, M, P> for L
where
    L: HasSinglyEnds<T, M, P>,
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
}
