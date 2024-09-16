use crate::{
    iter::{DoublyIterOwned, SinglyIterOwned},
    variant::Doubly,
    List, Singly,
};
use orx_selfref_col::MemoryPolicy;

impl<T, M> IntoIterator for List<Singly<T>, M>
where
    M: MemoryPolicy<Singly<T>>,
{
    type Item = T;

    type IntoIter = SinglyIterOwned<T>;

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

impl<T, M> IntoIterator for List<Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
{
    type Item = T;

    type IntoIter = DoublyIterOwned<T>;

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
