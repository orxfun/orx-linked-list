use crate::{
    iter::{DoublyIterOwned, SinglyIterOwned},
    variant::Doubly,
    List, Singly,
};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node};

impl<T, M, P> IntoIterator for List<Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
    type Item = T;

    type IntoIter = SinglyIterOwned<T, P>;

    /// Returns a consuming forward iterator to owned elements of the list from front to back.
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
    /// let mut iter = list.into_iter();
    ///
    /// assert_eq!(Some('a'), iter.next());
    /// assert_eq!(Some('b'), iter.next());
    /// assert_eq!(Some('c'), iter.next());
    /// assert!(iter.next().is_none());
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.0.into_inner().0)
    }
}

impl<T, M, P> IntoIterator for List<Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    type Item = T;

    type IntoIter = DoublyIterOwned<T, P>;

    /// Returns a consuming forward iterator to owned elements of the list from front to back.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// // a -> b -> c
    /// list.push_front('c');
    /// list.push_front('b');
    /// list.push_front('a');
    ///
    /// let mut iter = list.into_iter();
    ///
    /// assert_eq!(Some('a'), iter.next());
    /// assert_eq!(Some('b'), iter.next());
    /// assert_eq!(Some('c'), iter.next());
    /// assert!(iter.next().is_none());
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.0.into_inner().0)
    }
}
