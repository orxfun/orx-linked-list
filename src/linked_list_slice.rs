use crate::{LinkedList, LinkedListView};
use std::{fmt::Debug, ops::Deref};

pub struct LinkedListSlice<'s, 'a, T> {
    _list: &'s LinkedList<'a, T>,
    view: LinkedListView<'a, T>,
}

impl<'s, 'a, T> Deref for LinkedListSlice<'s, 'a, T> {
    type Target = LinkedListView<'a, T>;
    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

impl<'s, 'a, T> Clone for LinkedListSlice<'s, 'a, T> {
    fn clone(&self) -> Self {
        Self {
            _list: self._list.clone(),
            view: LinkedListView::new(
                self.view.len(),
                self.view.front_node(),
                self.view.back_node(),
            ),
        }
    }
}

impl<'s, 'a, T: Debug> Debug for LinkedListSlice<'s, 'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinkedListSlice")
            .field("view", &self.view)
            .finish()
    }
}

impl<'s, 'a, T> LinkedListSlice<'s, 'a, T> {
    // helpers
    pub(crate) fn new(list: &'s LinkedList<'a, T>, view: LinkedListView<'a, T>) -> Self {
        Self { _list: list, view }
    }
    pub(crate) fn new_with_view(&self, view: LinkedListView<'a, T>) -> Self {
        Self {
            _list: self._list,
            view,
        }
    }

    // splits
    /// Splits the linked list into two slices from the element at the given index.
    /// Returns None if `at > self.len()`.
    ///
    /// This operation should compute in O(n) time to locate the `at`-th element.
    ///
    /// Slices being only views on the linked list are cheap.
    ///
    /// Note that this method does **not** mutate the list; it rather returns two immutable views on two different parts of the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back('a');
    /// list.push_back('b');
    /// list.push_back('c');
    /// list.push_back('d');
    ///
    /// assert_eq!(list, &['a', 'b', 'c', 'd']);
    ///
    /// let (left, right) = list.split(1).unwrap();
    /// assert_eq!(left, &['a']);
    /// assert_eq!(right, &['b', 'c', 'd']);
    ///
    /// let (left, right) = list.split(0).unwrap();
    /// assert!(left.is_empty());
    /// assert_eq!(right, &['a', 'b', 'c', 'd']);
    ///
    /// let (left, right) = list.split(4).unwrap();
    /// assert_eq!(left, &['a', 'b', 'c', 'd']);
    /// assert!(right.is_empty());
    ///
    /// assert!(list.split(5).is_none());
    /// ```
    pub fn split(
        &self,
        at: usize,
    ) -> Option<(LinkedListSlice<'s, 'a, T>, LinkedListSlice<'s, 'a, T>)> {
        self.view_split(at)
            .map(|(a, b)| (self.new_with_view(a), self.new_with_view(b)))
    }

    /// Splits the linked list into the `front` and the remaining elements.
    /// Returns None if `self.is_empty()`.
    ///
    /// Note that this method does **not** mutate the list; it rather returns two immutable views on two different parts of the list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back('a');
    /// list.push_back('b');
    /// list.push_back('c');
    /// list.push_back('d');
    ///
    /// assert_eq!(list, &['a', 'b', 'c', 'd']);
    ///
    /// let (front, rest) = list.split_front().unwrap();
    ///
    /// assert_eq!(front, &'a');
    /// assert_eq!(rest, &['b', 'c', 'd']);
    /// ```
    pub fn split_front(&self) -> Option<(&T, LinkedListSlice<'s, 'a, T>)> {
        self.view_split_front()
            .map(|(x, y)| (x, self.new_with_view(y)))
    }

    /// Splits the linked list into elements until back and back.
    /// Returns None if `self.is_empty()`.
    ///
    /// Note that this method does **not** mutate the list; it rather returns two immutable views on two different parts of the list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back('a');
    /// list.push_back('b');
    /// list.push_back('c');
    /// list.push_back('d');
    ///
    /// assert_eq!(list, &['a', 'b', 'c', 'd']);
    ///
    /// let (rest, back) = list.split_back().unwrap();
    ///
    /// assert_eq!(back, &'d');
    /// assert_eq!(rest, &['a', 'b', 'c']);
    /// ```
    pub fn split_back(&self) -> Option<(LinkedListSlice<'s, 'a, T>, &T)> {
        self.view_split_back()
            .map(|(x, y)| (self.new_with_view(x), y))
    }
}

impl<'s, 'a, T: PartialEq> LinkedListSlice<'s, 'a, T> {
    /// Splits the linked list into two slices using the first from front element with the given `value` as the pivot:
    ///
    /// * left slice contains elements before the first appearance of the `value`,
    /// * right slice contains elements containing the `value` and all elements afterwards.
    ///
    /// Returns None if the value does not exist in the list.
    ///
    /// This operation should compute in O(n) time to locate the element with the given `value`.
    ///
    /// Slices being only views on the linked list are cheap.
    ///
    /// Note that this method does **not** mutate the list; it rather returns two immutable views on two different parts of the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back('a');
    /// list.push_back('b');
    /// list.push_back('c');
    /// list.push_back('d');
    ///
    /// assert_eq!(list, &['a', 'b', 'c', 'd']);
    ///
    /// let (left, right) = list.split_before(&'a').unwrap();
    /// assert!(left.is_empty());
    /// assert_eq!(right, &['a', 'b', 'c', 'd']);
    ///
    /// let (left, right) = list.split_before(&'b').unwrap();
    /// assert_eq!(left, &['a']);
    /// assert_eq!(right, &['b', 'c', 'd']);
    ///
    /// let (left, right) = list.split_before(&'d').unwrap();
    /// assert_eq!(left, &['a', 'b', 'c']);
    /// assert_eq!(right, &['d']);
    ///
    /// assert!(list.split_before(&'x').is_none());
    /// ```
    pub fn split_before(
        &self,
        value: &T,
    ) -> Option<(LinkedListSlice<'s, 'a, T>, LinkedListSlice<'s, 'a, T>)> {
        self.view_split_before(value)
            .map(|(a, b)| (self.new_with_view(a), self.new_with_view(b)))
    }

    /// Splits the linked list into two slices using the first from front element with the given `value` as the pivot:
    ///
    /// * left slice contains elements before the first appearance of the `value`,
    /// * right slice contains elements containing the `value` and all elements afterwards.
    ///
    /// Returns None if the value does not exist in the list.
    ///
    /// This operation should compute in O(n) time to locate the element with the given `value`.
    ///
    /// Slices being only views on the linked list are cheap.
    ///
    /// Note that this method does **not** mutate the list; it rather returns two immutable views on two different parts of the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back('a');
    /// list.push_back('b');
    /// list.push_back('c');
    /// list.push_back('d');
    ///
    /// assert_eq!(list, &['a', 'b', 'c', 'd']);
    ///
    /// let (left, right) = list.split_after(&'a').unwrap();
    /// assert_eq!(left, &['a']);
    /// assert_eq!(right, &['b', 'c', 'd']);
    ///
    /// let (left, right) = list.split_after(&'b').unwrap();
    /// assert_eq!(left, &['a', 'b']);
    /// assert_eq!(right, &['c', 'd']);
    ///
    /// let (left, right) = list.split_after(&'d').unwrap();
    /// assert_eq!(left, &['a', 'b', 'c', 'd']);
    /// assert!(right.is_empty());
    ///
    /// assert!(list.split_after(&'x').is_none());
    /// ```
    pub fn split_after(
        &self,
        value: &T,
    ) -> Option<(LinkedListSlice<'s, 'a, T>, LinkedListSlice<'s, 'a, T>)> {
        self.view_split_after(value)
            .map(|(a, b)| (self.new_with_view(a), self.new_with_view(b)))
    }
}
