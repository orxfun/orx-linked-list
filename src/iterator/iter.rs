use crate::{node::LinkedListNode, prelude::LinkedList};
use orx_imp_vec::prelude::PinnedVec;
use std::iter::FusedIterator;

impl<'a, T, P> LinkedList<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: 'a,
{
    /// Provides a forward iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(4);
    ///
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_front(0);
    /// list.push_back(3);
    ///
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter<'b>(&self) -> Iter<'b, T>
    where
        'a: 'b,
    {
        Iter {
            curr: self.imp[0].prev,
            len: self.len,
        }
    }
}

/// An iterator over the elements of a `LinkedList`.
///
/// This struct is created by `LinkedList::iter()` method.
pub struct Iter<'b, T> {
    curr: Option<&'b LinkedListNode<'b, T>>,
    len: usize,
}

impl<'b, T> Iterator for Iter<'b, T> {
    type Item = &'b T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr_node) = self.curr {
            self.curr = curr_node.next;
            self.len -= 1;
            curr_node.data.as_ref()
        } else {
            None
        }
    }
}
impl<T> FusedIterator for Iter<'_, T> {}
impl<T> ExactSizeIterator for Iter<'_, T> {
    fn len(&self) -> usize {
        self.len
    }
}
