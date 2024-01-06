use crate::{
    iter::{Iter, IterFromBack, IterFromFront, IterNodes},
    node::Node,
    LinkedList,
};
use orx_imp_vec::prelude::{SelfRefNext, SelfRefPrev};
use std::{cmp::Ordering, fmt::Debug};

/// An immutable slice of a linked list.
///
/// Note that this is not a slice in the `std::slice` sense in terms of the memory layout.
/// Nevertheless, it corresponds to a subset of elements of an original linked list, which itself is a linked list,
/// due to the recursiveness of the data structure.
pub struct LinkedListView<'a, T> {
    len: usize,
    front: Option<&'a Node<'a, T>>,
    back: Option<&'a Node<'a, T>>,
}

impl<'a, T> LinkedListView<'a, T> {
    /// Creates a new empty linked list slice.
    pub fn empty() -> Self {
        Default::default()
    }

    /// Returns the length of the linked list.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    /// assert_eq!(0, list.len());
    ///
    /// list.push_back('a');
    /// list.push_front('b');
    /// assert_eq!(2, list.len());
    ///
    /// _ = list.pop_back();
    /// assert_eq!(1, list.len());
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the LinkedList is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    /// assert!(list.is_empty());
    ///
    /// list.push_front('a');
    /// list.push_front('b');
    /// assert!(!list.is_empty());
    ///
    /// _ = list.pop_back();
    /// assert!(!list.is_empty());
    ///
    /// list.clear();
    /// assert!(list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Provides a reference to the back element, or None if the list is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.back(), None);
    ///
    /// list.push_back(42);
    /// assert_eq!(list.back(), Some(&42));
    ///
    /// list.push_front(1);
    /// assert_eq!(list.back(), Some(&42));
    ///
    /// list.push_back(7);
    /// assert_eq!(list.back(), Some(&7));
    /// ```
    pub fn back(&self) -> Option<&T> {
        self.back.map(|node| node.data_unchecked())
    }

    /// Provides a reference to the front element, or None if the list is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.front(), None);
    ///
    /// list.push_front(42);
    /// assert_eq!(list.front(), Some(&42));
    ///
    /// list.push_back(1);
    /// assert_eq!(list.front(), Some(&42));
    ///
    /// list.push_front(7);
    /// assert_eq!(list.front(), Some(&7));
    /// ```
    pub fn front(&self) -> Option<&T> {
        self.front.map(|node| node.data_unchecked())
    }

    /// Provides a forward iterator;
    /// which starts from the front-most element to the back.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
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
    pub fn iter(&self) -> Iter<'a, T, IterFromFront> {
        self.iter_nodes().into()
    }

    /// Provides a backward iterator;
    /// which starts from the back-most element to the front.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_front(0);
    /// list.push_back(3);
    ///
    /// let mut iter = list.iter_from_back();
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_from_back(&self) -> Iter<T, IterFromBack> {
        self.iter_nodes_from_back().into()
    }

    /// Returns a reference to element at the `at` position starting from the `front`;
    /// None when `at` is out of bounds.
    ///
    /// This operation requires *O*(*n*) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// // build linked list: a <-> b <-> c
    /// list.push_back('b');
    /// list.push_front('a');
    /// list.push_back('c');
    ///
    /// assert_eq!(Some(&'a'), list.get_at(0));
    /// assert_eq!(Some(&'b'), list.get_at(1));
    /// assert_eq!(Some(&'c'), list.get_at(2));
    /// assert_eq!(None, list.get_at(3));
    /// ```
    pub fn get_at(&self, at: usize) -> Option<&T> {
        self.get_node_at(at).map(|x| x.data_unchecked())
    }

    /// Collects the `LinkedListSlice` into a new `LinkedList`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let list: LinkedList<_> = (0..6).collect();
    /// assert_eq!(list, &[0, 1, 2, 3, 4, 5]);
    ///
    /// let (_, b) = list.split_after(&1).unwrap();
    /// assert_eq!(b, &[2, 3, 4, 5]); // b: LinkedListSlice is a view of the original list
    ///
    /// let mut new_list = b.collect(); // new_list: LinkedList is a new list built from b
    /// assert_eq!(new_list, &[2, 3, 4, 5]);
    ///
    /// new_list.pop_back();
    /// assert_eq!(new_list, &[2, 3, 4]);
    ///
    /// assert_eq!(list, &[0, 1, 2, 3, 4, 5]);
    /// assert_eq!(b, &[2, 3, 4, 5]);
    /// ```
    pub fn collect(&self) -> LinkedList<'a, T>
    where
        T: Clone,
    {
        self.iter().cloned().collect()
    }

    // helpers
    fn iter_nodes(&self) -> IterNodes<'a, T, IterFromFront> {
        IterNodes::new(self.len, self.front)
    }
    fn iter_nodes_from_back(&self) -> IterNodes<'a, T, IterFromBack> {
        IterNodes::new(self.len, self.back)
    }
    fn split_from_node(
        &self,
        node_idx: usize,
        node: &'a Node<'a, T>,
    ) -> (LinkedListView<'a, T>, LinkedListView<'a, T>) {
        let left = Self {
            front: node.prev().and(self.front),
            back: node.prev(),
            len: node_idx,
        };
        let right = Self {
            front: Some(node),
            back: self.back,
            len: self.len - node_idx,
        };
        (left, right)
    }

    // helpers - crate
    pub(crate) fn new(
        len: usize,
        front: Option<&'a Node<'a, T>>,
        back: Option<&'a Node<'a, T>>,
    ) -> Self {
        Self { len, back, front }
    }
    pub(crate) fn back_node(&self) -> Option<&'a Node<'a, T>> {
        self.back
    }
    pub(crate) fn front_node(&self) -> Option<&'a Node<'a, T>> {
        self.front
    }
    pub(crate) fn get_node_at(&self, at: usize) -> Option<&'a Node<'a, T>> {
        match at.cmp(&self.len) {
            Ordering::Less => {
                let at_from_back = self.len - at - 1;
                match at_from_back.cmp(&at) {
                    Ordering::Less => self.iter_nodes_from_back().nth(at_from_back),
                    _ => self.iter_nodes().nth(at),
                }
            }
            _ => None,
        }
    }
    /// Splits the linked list into two slices from the element at the given index.
    /// Returns None if `at > self.len()`.
    ///
    /// This operation should compute in O(n) time to locate the `at`-th element.
    ///
    /// Slices being only views on the linked list are cheap.
    ///
    /// Note that this method does **not** mutate the list; it rather returns two immutable views on two different parts of the list.
    pub(crate) fn view_split(
        &self,
        at: usize,
    ) -> Option<(LinkedListView<'a, T>, LinkedListView<'a, T>)> {
        if at == self.len {
            Some((Self::new(self.len, self.front, self.back), Self::empty()))
        } else {
            self.get_node_at(at)
                .map(|node| self.split_from_node(at, node))
        }
    }

    /// Splits the linked list into the `front` and the remaining elements.
    /// Returns None if `self.is_empty()`.
    ///
    /// Note that this method does **not** mutate the list; it rather returns two immutable views on two different parts of the list.
    pub(crate) fn view_split_front(&self) -> Option<(&T, LinkedListView<'a, T>)> {
        self.view_split(1)
            .map(|(x, y)| (x.front.expect("issome").data_unchecked(), y))
    }

    /// Splits the linked list into elements until back and back.
    /// Returns None if `self.is_empty()`.
    ///
    /// Note that this method does **not** mutate the list; it rather returns two immutable views on two different parts of the list.
    pub(crate) fn view_split_back(&self) -> Option<(LinkedListView<'a, T>, &T)> {
        if self.is_empty() {
            None
        } else {
            self.view_split(self.len - 1)
                .map(|(x, y)| (x, y.front.expect("issome").data_unchecked()))
        }
    }
}

impl<'a, T: PartialEq> LinkedListView<'a, T> {
    /// Returns whether or not the given `value` is in the list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// assert!(!list.contains(&'a'));
    ///
    /// list.push_back('a');
    /// assert!(list.contains(&'a'));
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        self.iter().any(|x| x == value)
    }

    /// Returns the index of the given `value` from the front of the list if it exists; None otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// assert!(list.index_of(&'a').is_none());
    ///
    /// list.push_back('a');
    /// list.push_back('b');
    /// list.push_front('c');
    /// list.push_front('d');
    /// assert_eq!(Some(2), list.index_of(&'a'));
    /// ```
    pub fn index_of(&self, value: &T) -> Option<usize> {
        self.index_and_node_of(value).map(|x| x.0)
    }

    /// Returns the index of the given `value` from the back of the list if it exists; None otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// assert!(list.from_back_index_of(&'a').is_none());
    ///
    /// list.push_back('a');
    /// list.push_back('b');
    /// list.push_front('c');
    /// list.push_front('d');
    /// assert_eq!(Some(1), list.from_back_index_of(&'a'));
    /// ```
    pub fn from_back_index_of(&self, value: &T) -> Option<usize> {
        self.iter_from_back()
            .enumerate()
            .find(|x| x.1 == value)
            .map(|x| x.0)
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
    pub(crate) fn view_split_before(
        &self,
        value: &T,
    ) -> Option<(LinkedListView<'a, T>, LinkedListView<'a, T>)> {
        self.index_and_node_of(value)
            .map(|(node_idx, node)| self.split_from_node(node_idx, node))
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
    pub(crate) fn view_split_after(
        &self,
        value: &T,
    ) -> Option<(LinkedListView<'a, T>, LinkedListView<'a, T>)> {
        self.index_and_node_of(value)
            .map(|(prev_idx, prev)| match prev.next() {
                None => (Self::new(self.len, self.front, self.back), Self::empty()),
                Some(node) => self.split_from_node(prev_idx + 1, node),
            })
    }

    // helpers
    #[inline(always)]
    fn index_and_node_of(&self, value: &T) -> Option<(usize, &'a Node<'a, T>)> {
        self.iter_nodes()
            .enumerate()
            .find(|x| x.1.data_unchecked() == value)
    }
}

impl<'a, T> Default for LinkedListView<'a, T> {
    fn default() -> Self {
        Self {
            len: 0,
            back: None,
            front: None,
        }
    }
}
impl<'a, T: Debug> Debug for LinkedListView<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinkedListSlice")
            .field("len", &self.len)
            .field("front", &self.front.map(|x| x.data_unchecked()))
            .field("back", &self.back.map(|x| x.data_unchecked()))
            .finish()
    }
}

#[cfg(test)]
pub(super) mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::{
        linked_list::{
            tests::{nodes, test_with_all_memory_utilization},
            LinkedList,
        },
        node::Node,
    };
    use orx_imp_vec::{
        prelude::{PinnedVec, SelfRefNext},
        ImpVec,
    };
    type Slice<'a, T> = LinkedListView<'a, T>;
    type List<'a, T> = LinkedList<'a, T>;

    fn validate_ref_get_idx<'a, T>(nodes: &ImpVec<Node<'a, T>>, node: &'a Node<'a, T>) -> usize {
        nodes.index_of(node).expect("invalid node reference")
    }
    fn ref_equals<'a, T>(
        nodes: &ImpVec<Node<'a, T>>,
        first: &'a Node<'a, T>,
        second: &'a Node<'a, T>,
    ) -> bool {
        validate_ref_get_idx(nodes, first) == validate_ref_get_idx(nodes, second)
    }
    pub(crate) fn validate_full_slice<'a, T>(list: &Slice<'a, T>, nodes: &ImpVec<Node<'a, T>>) {
        assert_eq!(
            list.len,
            nodes.iter().filter(|x| !x.is_closed()).count(),
            "list.len is not equal to number of active nodes"
        );

        let front_prev = list.front.and_then(|x| x.prev());
        assert!(front_prev.is_none(), "list.front.prev() is not none");

        let back_next = list.back.and_then(|x| x.next());
        assert!(back_next.is_none(), "list.back.next() is not none");

        validate_sub_slice(list, nodes);
    }
    fn validate_sub_slice<'a, T>(list: &Slice<'a, T>, nodes: &ImpVec<Node<'a, T>>) {
        if list.len == 0 {
            // empty case
            assert!(list.front.is_none(), "front.is_some() when list.is_empty()");
            assert!(list.back.is_none(), "back.is_some() when list.is_empty()");
        } else if list.len == 1 {
            // singleton case
            assert!(list.front.is_some(), "front.is_none() when len=1");
            assert!(list.back.is_some(), "back.is_none() when len=1");
            assert!(
                ref_equals(nodes, list.front.unwrap(), list.back.unwrap()),
                "front != back when len=1"
            );
        } else {
            // len >= 2 case
            assert!(list.front.is_some(), "front.is_none() when len>=2");
            assert!(list.back.is_some(), "back.is_none() when len>=2");
            assert!(
                !ref_equals(nodes, list.front.unwrap(), list.back.unwrap()),
                "front=back when len >= 2"
            );

            // forward
            let mut current = list.front.unwrap();
            for _ in 0..list.len - 1 {
                let next = current.next().expect("missing next");
                let next_prev = next.prev().expect("missing prev");
                assert!(ref_equals(nodes, current, next_prev), "next-prev mismatch");
                current = next;
            }
            assert!(
                ref_equals(nodes, current, list.back.unwrap()),
                "failed to reach back in forward iteration"
            );

            // backward
            let mut current = list.back.unwrap();
            for _ in 0..list.len - 1 {
                let prev = current.prev().expect("missing prev");
                let prev_next = prev.next().expect("missing next");
                assert!(ref_equals(nodes, current, prev_next), "next-prev mismatch");
                current = prev;
            }
            assert!(
                ref_equals(nodes, current, list.front.unwrap()),
                "failed to reach front in backward iteration"
            );
        }
    }

    pub(crate) fn collect_vec<T: Clone>(list: &Slice<'_, T>) -> Vec<T> {
        list.iter().cloned().collect()
    }

    #[test]
    fn empty() {
        let slice: Slice<'_, char> = Slice::empty();

        assert_eq!(0, slice.len());
        assert!(slice.is_empty());
        assert_eq!(None, slice.back());
        assert_eq!(None, slice.front());

        assert_eq!(0, slice.iter().count());
        assert_eq!(0, slice.iter_from_back().count());

        assert_eq!(None, slice.get_at(0));

        let (left, right) = slice.view_split(0).expect("is-some");
        assert!(left.is_empty());
        assert!(right.is_empty());

        assert!(slice.view_split(1).is_none());
    }

    #[test]
    fn len_isempty() {
        let node = Node::active('a', None, None);

        let slice = Slice::new(1, Some(&node), Some(&node));
        assert_eq!(1, slice.len());
        assert!(!slice.is_empty());

        let slice: Slice<char> = Slice::new(0, None, None);
        assert_eq!(0, slice.len());
        assert!(slice.is_empty());

        let slice: Slice<char> = Slice::empty();
        assert_eq!(0, slice.len());
        assert!(slice.is_empty());
    }

    #[test]
    fn back_front() {
        let slice: Slice<char> = Slice::empty();
        assert!(slice.back().is_none());
        assert!(slice.front().is_none());

        let node1 = Node::active('a', None, None);

        let slice1 = Slice::new(1, Some(&node1), Some(&node1));
        assert_eq!(Some(&'a'), slice1.back_node().map(|x| x.data_unchecked()));
        assert_eq!(Some(&'a'), slice1.back());
        assert_eq!(Some(&'a'), slice1.front_node().map(|x| x.data_unchecked()));
        assert_eq!(Some(&'a'), slice1.front());

        let node2 = Node::active('b', None, None);

        let slice2 = Slice::new(2, Some(&node1), Some(&node2));
        assert_eq!(Some(&'a'), slice2.front_node().map(|x| x.data_unchecked()));
        assert_eq!(Some(&'a'), slice2.front());
        assert_eq!(Some(&'b'), slice2.back_node().map(|x| x.data_unchecked()));
        assert_eq!(Some(&'b'), slice2.back());
    }

    #[test]
    fn iter() {
        let slice: Slice<char> = Slice::empty();
        assert_eq!(0, slice.iter().count());
        assert_eq!(0, slice.iter_from_back().count());

        let node = Node::active('a', None, None);
        let slice = Slice::new(1, Some(&node), Some(&node));
        assert_eq!(1, slice.iter().count());
        assert_eq!(1, slice.iter_from_back().count());

        let mut vec = vec!['a', 'b', 'c', 'd'];

        let node3 = Node::active(vec[3], None, None);
        let node2 = Node::active(vec[2], None, Some(&node3));
        let node1 = Node::active(vec[1], None, Some(&node2));
        let node0 = Node::active(vec[0], None, Some(&node1));
        let slice = Slice::new(vec.len(), Some(&node0), Some(&node3));
        let collect: Vec<_> = slice.iter().copied().collect();
        assert_eq!(&vec, &collect);

        let node0 = Node::active(vec[0], None, None);
        let node1 = Node::active(vec[1], Some(&node0), None);
        let node2 = Node::active(vec[2], Some(&node1), None);
        let node3 = Node::active(vec[3], Some(&node2), None);
        let slice = Slice::new(vec.len(), Some(&node0), Some(&node3));
        let collect: Vec<_> = slice.iter_from_back().copied().collect();
        vec.reverse();
        assert_eq!(&vec, &collect);
    }

    #[test]
    fn get_at_trivial() {
        let slice: Slice<char> = Slice::empty();
        assert!(slice.get_node_at(0).is_none());
        assert!(slice.get_at(0).is_none());

        let node = Node::active('a', None, None);
        let slice = Slice::new(1, Some(&node), Some(&node));
        assert_eq!(Some(&'a'), slice.get_at(0));
        assert_eq!(Some(&'a'), slice.get_node_at(0).map(|x| x.data_unchecked()));
        assert!(slice.get_node_at(1).is_none());
        assert!(slice.get_at(1).is_none());
    }

    #[test]
    fn get_at_front() {
        let vec = vec!['a', 'b', 'c', 'd'];

        let node3 = Node::active(vec[3], None, None);
        let node2 = Node::active(vec[2], None, Some(&node3));
        let node1 = Node::active(vec[1], None, Some(&node2));
        let node0 = Node::active(vec[0], None, Some(&node1));
        let slice = Slice::new(vec.len(), Some(&node0), Some(&node3));

        assert_eq!(Some(&'a'), slice.get_at(0));
        assert_eq!(Some(&'b'), slice.get_at(1));
    }

    #[test]
    #[should_panic]
    fn get_at_front_not_used_when_farter() {
        let vec = vec!['a', 'b', 'c', 'd'];

        let node3 = Node::active(vec[3], None, None);
        let node2 = Node::active(vec[2], None, Some(&node3));
        let node1 = Node::active(vec[1], None, Some(&node2));
        let node0 = Node::active(vec[0], None, Some(&node1));
        let slice = Slice::new(vec.len(), Some(&node0), Some(&node3));

        assert_eq!(Some(&'c'), slice.get_at(2));
    }

    #[test]
    fn get_at_back() {
        let vec = vec!['a', 'b', 'c', 'd'];

        let node0 = Node::active(vec[0], None, None);
        let node1 = Node::active(vec[1], Some(&node0), None);
        let node2 = Node::active(vec[2], Some(&node1), None);
        let node3 = Node::active(vec[3], Some(&node2), None);
        let slice = Slice::new(vec.len(), Some(&node0), Some(&node3));

        assert_eq!(Some(&'c'), slice.get_at(2));
        assert_eq!(Some(&'d'), slice.get_at(3));
    }

    #[test]
    #[should_panic]
    fn get_at_back_not_used_when_farter() {
        let vec = vec!['a', 'b', 'c', 'd'];

        let node0 = Node::active(vec[0], None, None);
        let node1 = Node::active(vec[1], Some(&node0), None);
        let node2 = Node::active(vec[2], Some(&node1), None);
        let node3 = Node::active(vec[3], Some(&node2), None);
        let slice = Slice::new(vec.len(), Some(&node0), Some(&node3));

        assert_eq!(Some(&'b'), slice.get_at(1));
    }

    #[test]
    fn get_at_out_of_bounds() {
        let vec = vec!['a', 'b', 'c', 'd'];

        let node3 = Node::active(vec[3], None, None);
        let node2 = Node::active(vec[2], None, Some(&node3));
        let node1 = Node::active(vec[1], None, Some(&node2));
        let node0 = Node::active(vec[0], None, Some(&node1));
        let slice = Slice::new(vec.len(), Some(&node0), Some(&node3));

        assert_eq!(None, slice.get_at(4));
    }

    #[test]
    fn split() {
        fn test(mut list: List<char>) {
            list.push_back('a');
            list.push_back('b');
            list.push_back('c');
            list.push_back('d');

            let nodes = nodes(&list);

            let (a, b) = list.split(0).unwrap();
            validate_sub_slice(&a, nodes);
            validate_sub_slice(&b, nodes);
            assert_eq!(Vec::<char>::new(), collect_vec(&a));
            assert_eq!(vec!['a', 'b', 'c', 'd'], collect_vec(&b));

            let (a, b) = list.split(1).unwrap();
            validate_sub_slice(&a, nodes);
            validate_sub_slice(&b, nodes);
            assert_eq!(vec!['a'], collect_vec(&a));
            assert_eq!(vec!['b', 'c', 'd'], collect_vec(&b));

            let (a, b) = list.split(2).unwrap();
            validate_sub_slice(&a, nodes);
            validate_sub_slice(&b, nodes);
            assert_eq!(vec!['a', 'b'], collect_vec(&a));
            assert_eq!(vec!['c', 'd'], collect_vec(&b));

            let (a, b) = list.split(3).unwrap();
            validate_sub_slice(&a, nodes);
            validate_sub_slice(&b, nodes);
            assert_eq!(vec!['a', 'b', 'c'], collect_vec(&a));
            assert_eq!(vec!['d'], collect_vec(&b));

            let (a, b) = list.split(4).unwrap();
            validate_sub_slice(&a, nodes);
            validate_sub_slice(&b, nodes);
            assert_eq!(vec!['a', 'b', 'c', 'd'], collect_vec(&a));
            assert_eq!(Vec::<char>::new(), collect_vec(&b));

            assert!(list.split(5).is_none());
        }

        test_with_all_memory_utilization(test);
    }

    #[test]
    fn split_front_back() {
        fn test(mut list: List<char>) {
            assert!(list.split_front().is_none());
            assert!(list.split_back().is_none());

            list.push_back('a');

            let (front, rest) = list.split_front().unwrap();
            assert_eq!(&'a', front);
            assert!(rest.is_empty());

            let (rest, back) = list.split_back().unwrap();
            assert!(rest.is_empty());
            assert_eq!(&'a', back);

            list.push_back('b');

            let (front, rest) = list.split_front().unwrap();
            assert_eq!(front, &'a');
            assert_eq!(rest, &['b']);

            let (rest, back) = list.split_back().unwrap();
            assert_eq!(rest, &['a']);
            assert_eq!(back, &'b');

            list.push_front('x');
            list.push_back('y');

            let (front, rest) = list.split_front().unwrap();
            assert_eq!(front, &'x');
            assert_eq!(rest, &['a', 'b', 'y']);

            let (rest, back) = list.split_back().unwrap();
            assert_eq!(rest, &['x', 'a', 'b']);
            assert_eq!(back, &'y');
        }

        test_with_all_memory_utilization(test);
    }

    #[test]
    fn contains_index() {
        fn test(mut list: List<char>) {
            let vec = vec!['a', 'b', 'c', 'd'];

            list.push_back('a');
            list.push_back('b');
            list.push_back('c');
            list.push_back('d');

            for (i, x) in vec.iter().enumerate() {
                assert!(list.contains(x));
                assert_eq!(Some(i), list.index_of(x));
                assert_eq!(Some(vec.len() - 1 - i), list.from_back_index_of(x));
            }

            assert!(!list.contains(&'e'));
            assert!(list.index_of(&'e').is_none());
            assert!(list.from_back_index_of(&'e').is_none());
        }

        test_with_all_memory_utilization(test);
    }

    #[test]
    fn split_before_after_multiple_matches() {
        fn test(mut list: List<char>) {
            list.push_back('a');
            list.push_back('b');
            list.push_back('c');
            list.push_back('d');
            list.push_back('e');
            list.push_back('c');
            list.push_back('g');

            let (a, b) = list.view_split_after(&'c').unwrap();

            assert_eq!(a, &['a', 'b', 'c']);
            assert_eq!(b, &['d', 'e', 'c', 'g']);

            let (a, b) = list.view_split_before(&'c').unwrap();

            assert_eq!(a, &['a', 'b']);
            assert_eq!(b, &['c', 'd', 'e', 'c', 'g']);
        }

        test_with_all_memory_utilization(test);
    }

    #[test]
    fn split_before_after() {
        fn test(mut list: List<usize>) {
            let vec: Vec<_> = (0..185).collect();
            for c in &vec {
                list.push_back(*c);
            }

            for (i, c) in vec.iter().enumerate() {
                let (a, b) = list.view_split_after(c).unwrap();
                assert_eq!(a, &vec[0..=i]);
                assert_eq!(b, &vec[(i + 1)..]);

                let (a, b) = list.view_split_before(c).unwrap();
                assert_eq!(a, &vec[0..i]);
                assert_eq!(b, &vec[i..]);
            }
        }

        test_with_all_memory_utilization(test);
    }
}
