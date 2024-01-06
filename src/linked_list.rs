use crate::{
    linked_list_slice::LinkedListSlice,
    linked_list_view::LinkedListView,
    mem::{reclaim_memory, MemoryStatus, MemoryUtilization},
    node::Node,
};
use orx_imp_vec::{
    prelude::{PinnedVec, SelfRefNext, SelfRefPrev},
    ImpVec,
};
use std::{fmt::Debug, ops::Deref};

/// The LinkedList allows pushing and popping elements at either end in constant time.
pub struct LinkedList<'a, T> {
    vec: ImpVec<Node<'a, T>>,
    slice: LinkedListView<'a, T>,
    memory_utilization: MemoryUtilization,
}

impl<'a, T> Deref for LinkedList<'a, T> {
    type Target = LinkedListView<'a, T>;
    fn deref(&self) -> &Self::Target {
        &self.slice
    }
}
impl<'a, T> Default for LinkedList<'a, T> {
    fn default() -> Self {
        Self {
            vec: ImpVec::default(),
            slice: Default::default(),
            memory_utilization: Default::default(),
        }
    }
}
impl<'a, T: Debug> Debug for LinkedList<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinkedList")
            .field("slice", &self.slice)
            .field("memory_utilization", &self.memory_utilization)
            .finish()
    }
}
impl<'a, T: Clone> Clone for LinkedList<'a, T> {
    fn clone(&self) -> Self {
        Self::from_iter(self.iter().cloned()).with_memory_utilization(self.memory_utilization)
    }
}

impl<'a, T> LinkedList<'a, T> {
    /// Creates a new empty linked list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Converts the linked list into one with the given `memory_utilization`.
    pub fn with_memory_utilization(self, memory_utilization: MemoryUtilization) -> Self {
        Self {
            vec: self.vec,
            slice: self.slice,
            memory_utilization,
        }
    }

    /// Returns the memory utilization settings of the list.
    pub fn memory_utilization(&self) -> MemoryUtilization {
        self.memory_utilization
    }

    /// Returns a reference to the entire list as an immutable slice.
    pub fn as_slice(&self) -> LinkedListSlice<'_, 'a, T> {
        let view = LinkedListView::new(
            self.slice.len(),
            self.slice.front_node(),
            self.slice.back_node(),
        );
        LinkedListSlice::new(self, view)
    }

    /// Provides a mutable reference to the back element, or None if the list is empty.
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
    /// match list.back_mut() {
    ///     None => {},
    ///     Some(x) => *x = 7,
    /// }
    /// assert_eq!(list.back(), Some(&7));
    /// ```
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.back_node().map(|n| self.data_mut(n))
    }

    /// Provides a mutable reference to the front element, or None if the list is empty.
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
    /// match list.front_mut() {
    ///     None => {},
    ///     Some(x) => *x = 7,
    /// }
    /// assert_eq!(list.front(), Some(&7));
    /// ```
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.front_node().map(|n| self.data_mut(n))
    }

    /// Returns a mutable reference to element at the `at` position starting from the `front`;
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
    /// *list.get_at_mut(0).unwrap() = 'x';
    /// *list.get_at_mut(1).unwrap() = 'y';
    /// *list.get_at_mut(2).unwrap() = 'z';
    /// assert_eq!(None, list.get_at_mut(3));
    ///
    /// assert_eq!(Some(&'x'), list.get_at(0));
    /// assert_eq!(Some(&'y'), list.get_at(1));
    /// assert_eq!(Some(&'z'), list.get_at(2));
    /// ```
    pub fn get_at_mut(&mut self, at: usize) -> Option<&mut T> {
        self.get_node_at(at).map(|n| self.data_mut(n))
    }

    /// Appends an element to the back of a list.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// list.push_back('a');
    /// assert_eq!('a', *list.back().unwrap());
    ///
    /// list.push_back('b');
    /// assert_eq!('b', *list.back().unwrap());
    ///
    /// list.push_front('x');
    /// assert_eq!('b', *list.back().unwrap());
    /// ```
    pub fn push_back(&mut self, value: T) {
        match self.back_node() {
            None => self.push_first_node(value),
            Some(old_back) => {
                let node = Node::active(value, Some(old_back), None);
                let back = unsafe { self.vec.push_get_ref(node) };
                self.vec.set_next(old_back, Some(back));
                self.slice = LinkedListView::new(self.len() + 1, self.front_node(), Some(back));
            }
        }
    }

    /// Appends an element to the back of a list.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// list.push_front('a');
    /// assert_eq!('a', *list.front().unwrap());
    ///
    /// list.push_front('b');
    /// assert_eq!('b', *list.front().unwrap());
    ///
    /// list.push_back('x');
    /// assert_eq!('b', *list.front().unwrap());
    /// ```
    pub fn push_front(&mut self, value: T) {
        match self.front_node() {
            None => self.push_first_node(value),
            Some(old_front) => {
                let node = Node::active(value, None, Some(old_front));
                let front = unsafe { self.vec.push_get_ref(node) };
                self.vec.set_prev(old_front, Some(front));
                self.slice = LinkedListView::new(self.len() + 1, Some(front), self.back_node());
            }
        }
    }

    /// Removes the last element from a list and returns it, or None if it is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// // build linked list: x <-> a <-> b <-> c
    /// list.push_back('a');
    /// list.push_back('b');
    /// list.push_front('x');
    /// list.push_back('c');
    ///
    /// assert_eq!(Some('c'), list.pop_back());
    /// assert_eq!(Some('b'), list.pop_back());
    /// assert_eq!(Some('a'), list.pop_back());
    /// assert_eq!(Some('x'), list.pop_back());
    /// assert_eq!(None, list.pop_back());
    /// assert_eq!(None, list.pop_front());
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        match self.back_node() {
            None => None,
            Some(old_back) => {
                let back = old_back.prev();
                self.slice = LinkedListView::new(self.len() - 1, back.and(self.front_node()), back);
                Some(self.close_node(old_back))
            }
        }
    }

    /// Removes the last element from a list and returns it, or None if it is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// // build linked list: c <-> b <-> a <-> x
    /// list.push_front('a');
    /// list.push_front('b');
    /// list.push_back('x');
    /// list.push_front('c');
    ///
    /// assert_eq!(Some('c'), list.pop_front());
    /// assert_eq!(Some('b'), list.pop_front());
    /// assert_eq!(Some('a'), list.pop_front());
    /// assert_eq!(Some('x'), list.pop_front());
    /// assert_eq!(None, list.pop_front());
    /// assert_eq!(None, list.pop_back());
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        match self.front_node() {
            None => None,
            Some(old_front) => {
                let front = old_front.next();
                self.slice =
                    LinkedListView::new(self.len() - 1, front, front.and(self.back_node()));
                Some(self.close_node(old_front))
            }
        }
    }

    /// Removes all elements from the LinkedList.
    ///
    /// This operation should compute in O(n) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// // build linked list: x <-> a <-> b <-> c
    /// list.push_front('a');
    /// list.push_front('b');
    /// list.push_back('x');
    /// list.push_front('c');
    ///
    /// assert_eq!(4, list.len());
    /// assert_eq!(Some(&'c'), list.front());
    /// assert_eq!(Some(&'x'), list.back());
    ///
    /// list.clear();
    ///
    /// assert!(list.is_empty());
    /// assert_eq!(None, list.front());
    /// assert_eq!(None, list.back());
    /// ```
    pub fn clear(&mut self) {
        self.vec.clear();
        self.slice = LinkedListView::empty();
    }

    /// Removes the element at the given index and returns it; returns None if `at` is out of bounds.
    ///
    /// This operation requires *O*(*n*) time to access the `at`-th element and constant time to remove.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// // build linked list: x <-> a <-> b <-> c
    /// list.push_back('a');
    /// list.push_back('b');
    /// list.push_front('x');
    /// list.push_back('c');
    ///
    /// assert_eq!(list.remove_at(1), Some('a'));
    /// assert_eq!(list.remove_at(0), Some('x'));
    /// assert_eq!(list.remove_at(1), Some('c'));
    /// assert_eq!(list.remove_at(0), Some('b'));
    /// ```
    pub fn remove_at(&mut self, at: usize) -> Option<T> {
        self.get_node_at(at).map(|node| {
            let (front, back) = match (at, self.len()) {
                (_, 1) => (None, None),
                (0, _) => (node.next(), self.back_node()),
                (at, _) if at == self.len() - 1 => (self.front_node(), node.prev()),
                _ => (self.front_node(), self.back_node()),
            };
            self.slice = LinkedListView::new(self.len() - 1, front, back);
            self.close_node(node)
        })
    }

    /// Inserts the element at the given position.
    ///
    /// This operation requires *O*(*n*) time to access the `at`-th element and constant time to insert.
    ///
    /// # Panics
    /// Panics if at > len
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// // build linked list: a <-> b <-> c
    /// list.push_back('a');
    /// list.push_back('b');
    /// list.push_back('c');
    ///
    /// list.insert_at(1, 'w');
    /// assert_eq!(vec!['a', 'w', 'b', 'c'], list.iter().copied().collect::<Vec<_>>());
    /// ```
    pub fn insert_at(&mut self, at: usize, value: T) {
        match at {
            0 => self.push_front(value),
            at if at == self.len() => self.push_back(value),
            at => {
                let next = self.get_node_at(at).expect("out-of-bound!");
                let prev = next.prev().expect("issome since at is not 0 or n-1");
                let node = Node::active(value, Some(prev), Some(next));
                let noderef = unsafe { self.vec.push_get_ref(node) };
                self.vec.set_prev(next, Some(noderef));
                self.vec.set_next(prev, Some(noderef));
                self.slice =
                    LinkedListView::new(self.len() + 1, self.front_node(), self.back_node());
            }
        }
    }

    // mem
    /// Returns the memory utilization status of the linked list's underlying storage.
    pub fn memory_status(&self) -> MemoryStatus {
        MemoryStatus::of_list(self.len(), self.vec.len())
    }

    /// This method reclaims the gaps which are opened due to lazy pops and removals,
    /// and brings back `memory_status` to 100% in *O(n)* time complexity.
    ///
    /// Memory can be reclaimed by calling this method manually.
    /// On the other hand, `memory_utilization` settings of the list automatically calls this method with different frequencies:
    ///
    /// * `MemoryUtilization::WithThreshold(x)` => calls whenever the utilization falls lower than x => recommended setting with x ~= 0.7.
    /// * `MemoryUtilization::Lazy` => never calls => memory utilization might be low if there are many pops & removals.
    /// * `MemoryUtilization::Eager` => calls after every pop & removal => keeps utilization at 100% but leads to linear time pops & removals which significantly slows down the process.
    pub fn reclaim_memory(&mut self) {
        reclaim_memory(self, self.vec.len());
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
    ) -> Option<(LinkedListSlice<'_, 'a, T>, LinkedListSlice<'_, 'a, T>)> {
        self.as_slice().split(at)
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
    pub fn split_front(&self) -> Option<(&T, LinkedListSlice<'_, 'a, T>)> {
        self.view_split_front()
            .map(|(x, y)| (x, self.as_slice().new_with_view(y)))
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
    pub fn split_back(&self) -> Option<(LinkedListSlice<'_, 'a, T>, &T)> {
        self.view_split_back()
            .map(|(x, y)| (self.as_slice().new_with_view(x), y))
    }

    // helpers
    fn push_first_node(&mut self, value: T) {
        debug_assert!(self.is_empty());

        let node = unsafe { self.vec.push_get_ref(Node::active(value, None, None)) };
        self.slice = LinkedListView::new(1, Some(node), Some(node));
    }
    fn data_mut(&mut self, node: &Node<'a, T>) -> &mut T {
        let idx = self
            .vec
            .index_of(node)
            .expect("issome -> node exists idx must exist");
        self.vec[idx]
            .data_mut()
            .expect("issome -> node exists, cannot be a vacant node")
    }

    fn close_node(&mut self, node: &Node<'a, T>) -> T {
        if let Some(prev) = node.prev() {
            self.vec.set_next(prev, node.next());
        }

        if let Some(next) = node.next() {
            self.vec.set_prev(next, node.prev());
        }

        let node_idx = self.vec.index_of(node).expect("issome");
        let data: Option<T> = self
            .vec
            .replace_at(node_idx, Node::closed())
            .expect("issome node exists")
            .into();

        self.memory_utilization.reclaim(self, self.vec.len());

        data.expect("issome data exists")
    }

    // helpers - mem
    pub(crate) fn is_vacant(&self, idx: usize) -> bool {
        self.vec[idx].is_closed()
    }
    pub(crate) fn move_to_vacant_node(&mut self, occupied_idx: usize, vacant_idx: usize) {
        debug_assert!(!self.is_vacant(occupied_idx));
        debug_assert!(self.is_vacant(vacant_idx));

        let (_, node) = unsafe {
            self.vec
                .move_get_ref(occupied_idx, vacant_idx, Node::closed())
        }
        .expect("issome");

        if let Some(prev) = node.prev() {
            self.vec.set_next(prev, Some(node));
        } else {
            self.slice = LinkedListView::new(self.len(), Some(node), self.back_node());
        }

        if let Some(next) = node.next() {
            self.vec.set_prev(next, Some(node));
        } else {
            self.slice = LinkedListView::new(self.len(), self.front_node(), Some(node))
        }
    }
    pub(crate) fn truncate_vec(&mut self) {
        debug_assert!(self.vec.iter().take(self.len()).all(|x| !x.is_closed()));
        debug_assert!(self.vec.iter().skip(self.len()).all(|x| x.is_closed()));

        let len = self.len();
        unsafe { self.vec.unsafe_truncate(len) };
    }
}

impl<'a, T: PartialEq> LinkedList<'a, T> {
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
    ) -> Option<(LinkedListSlice<'_, 'a, T>, LinkedListSlice<'_, 'a, T>)> {
        self.as_slice().split_before(value)
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
    ) -> Option<(LinkedListSlice<'_, 'a, T>, LinkedListSlice<'_, 'a, T>)> {
        self.as_slice().split_after(value)
    }
}

#[cfg(test)]
pub(super) mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::linked_list_view::tests::validate_full_slice;
    type List<'a, T> = LinkedList<'a, T>;

    pub(crate) fn test_with_all_memory_utilization<'a, T, A>(mut assert: A)
    where
        T: 'a,
        A: FnMut(List<'a, T>) + 'a,
    {
        assert(List::new());
        assert(List::default());
        assert(List::new().with_memory_utilization(MemoryUtilization::Eager));
        assert(List::new().with_memory_utilization(MemoryUtilization::Lazy));
        assert(List::new().with_memory_utilization(MemoryUtilization::WithThreshold(0.75)));
    }
    pub(crate) fn storage_to_datavec<T: Clone>(list: &List<T>) -> Vec<Option<T>> {
        list.vec.iter().map(|x| x.data().cloned()).collect()
    }
    pub(crate) fn to_vec<T: Clone>(list: &List<T>) -> Vec<T> {
        list.iter().cloned().collect()
    }
    fn to_vec_from_back<T: Clone>(list: &List<T>) -> Vec<T> {
        list.iter_from_back().cloned().collect()
    }
    pub(crate) fn nodes<'a, 'b, T>(list: &'b List<'a, T>) -> &'b ImpVec<Node<'a, T>>
    where
        'a: 'b,
    {
        &list.vec
    }

    #[test]
    fn new() {
        fn test(mut list: List<char>) {
            validate_full_slice(&list, &list.vec);
            assert!(list.get_at_mut(0).is_none());

            assert!(list.pop_back().is_none());
            assert!(list.pop_front().is_none());
            assert!(list.remove_at(0).is_none());
        }

        test_with_all_memory_utilization(test);

        let mut list = List::new();
        list.push_back('a');
        list.push_front('b');
        list.clear();
        test(list);
    }

    #[test]
    fn clone() {
        let mut list = LinkedList::from_iter(['a', 'b', 'c'].into_iter());
        let clone = list.clone();

        validate_full_slice(&list, &list.vec);
        validate_full_slice(&clone, &clone.vec);
        assert_eq!(list, &['a', 'b', 'c']);
        assert_eq!(clone, &['a', 'b', 'c']);
        assert_eq!(list, clone);

        _ = list.pop_back();

        validate_full_slice(&list, &list.vec);
        validate_full_slice(&clone, &clone.vec);
        assert_eq!(list, &['a', 'b']);
        assert_eq!(clone, &['a', 'b', 'c']);
        assert_ne!(list, clone);

        list.clear();
        list.push_back('d');

        validate_full_slice(&list, &list.vec);
        validate_full_slice(&clone, &clone.vec);
        assert_eq!(list, &['d']);
        assert_eq!(clone, &['a', 'b', 'c']);
        assert_ne!(list, clone);

        drop(list);

        validate_full_slice(&clone, &clone.vec);
        assert_eq!(clone, &['a', 'b', 'c']);
    }

    #[test]
    fn with_utilization() {
        let list: List<char> = List::default();
        assert_eq!(list.memory_utilization(), MemoryUtilization::default());

        let list: List<char> = List::new();
        assert_eq!(list.memory_utilization(), MemoryUtilization::default());

        let list: List<char> = List::new().with_memory_utilization(MemoryUtilization::Eager);
        assert_eq!(list.memory_utilization(), MemoryUtilization::Eager);

        let list: List<char> = List::new().with_memory_utilization(MemoryUtilization::Lazy);
        assert_eq!(list.memory_utilization(), MemoryUtilization::Lazy);

        let list: List<char> =
            List::new().with_memory_utilization(MemoryUtilization::WithThreshold(0.24));
        assert_eq!(
            list.memory_utilization(),
            MemoryUtilization::WithThreshold(0.24)
        );
    }

    #[test]
    fn front_back_mut() {
        fn test(mut list: List<char>) {
            assert!(list.front_mut().is_none());
            assert!(list.back_mut().is_none());

            list.push_back('a');
            validate_full_slice(&list, &list.vec);

            *list.front_mut().unwrap() = 'x';
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some(&'x'), list.front());
            assert_eq!(Some(&'x'), list.back());

            *list.back_mut().unwrap() = 'y';
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some(&'y'), list.front());
            assert_eq!(Some(&'y'), list.back());

            list.push_front('x');
            validate_full_slice(&list, &list.vec);

            *list.front_mut().unwrap() = 'a';
            *list.back_mut().unwrap() = 'b';

            assert_eq!(Some(&'a'), list.front());
            assert_eq!(Some(&'b'), list.back());

            validate_full_slice(&list, &list.vec);
        }

        test_with_all_memory_utilization(test);
    }

    #[test]
    fn get_at_mut() {
        fn test(mut list: List<char>) {
            assert!(list.get_at_mut(0).is_none());

            list.push_back('a');

            *list.get_at_mut(0).unwrap() = 'x';
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some(&'x'), list.front());
            assert_eq!(Some(&'x'), list.back());
            assert_eq!(Some(&'x'), list.get_at(0));

            list.push_front('y');

            *list.get_at_mut(0).unwrap() = 'a';
            *list.get_at_mut(1).unwrap() = 'b';
            validate_full_slice(&list, &list.vec);

            assert_eq!(Some(&'a'), list.front());
            assert_eq!(Some(&'b'), list.back());
            assert_eq!(Some(&'a'), list.get_at(0));
            assert_eq!(Some(&'b'), list.get_at(1));
        }

        test_with_all_memory_utilization(test);
    }

    #[test]
    fn push_back() {
        fn test(mut list: List<char>) {
            list.push_back('a');
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some(&'a'), list.back());
            assert_eq!(Some(&'a'), list.front());

            list.push_back('b');
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some(&'b'), list.back());
            assert_eq!(
                &'a',
                list.back_node().unwrap().prev().unwrap().data_unchecked()
            );
            assert_eq!(Some(&'a'), list.front());
            assert_eq!(
                &'b',
                list.front_node().unwrap().next().unwrap().data_unchecked()
            );

            list.push_back('c');
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some(&'c'), list.back());
            assert_eq!(
                &'b',
                list.back_node().unwrap().prev().unwrap().data_unchecked()
            );
            assert_eq!(
                &'a',
                list.back_node()
                    .unwrap()
                    .prev()
                    .unwrap()
                    .prev()
                    .unwrap()
                    .data_unchecked()
            );
            assert_eq!(Some(&'a'), list.front());
            assert_eq!(
                &'b',
                list.front_node().unwrap().next().unwrap().data_unchecked()
            );
            assert_eq!(
                &'c',
                list.front_node()
                    .unwrap()
                    .next()
                    .unwrap()
                    .next()
                    .unwrap()
                    .data_unchecked()
            );

            assert_eq!(
                vec!['a', 'b', 'c'],
                list.iter().copied().collect::<Vec<_>>()
            );

            assert_eq!(
                vec!['c', 'b', 'a'],
                list.iter_from_back().copied().collect::<Vec<_>>()
            );
        }
        test_with_all_memory_utilization(test);
    }

    #[test]
    fn push_front() {
        fn test(mut list: List<char>) {
            list.push_front('a');
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some(&'a'), list.back());
            assert_eq!(Some(&'a'), list.front());

            list.push_front('b');
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some(&'a'), list.back());
            assert_eq!(
                &'b',
                list.back_node().unwrap().prev().unwrap().data_unchecked()
            );
            assert_eq!(Some(&'b'), list.front());
            assert_eq!(
                &'a',
                list.front_node().unwrap().next().unwrap().data_unchecked()
            );

            list.push_front('c');
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some(&'a'), list.back());
            assert_eq!(
                &'b',
                list.back_node().unwrap().prev().unwrap().data_unchecked()
            );
            assert_eq!(
                &'c',
                list.back_node()
                    .unwrap()
                    .prev()
                    .unwrap()
                    .prev()
                    .unwrap()
                    .data_unchecked()
            );
            assert_eq!(Some(&'c'), list.front());
            assert_eq!(
                &'b',
                list.front_node().unwrap().next().unwrap().data_unchecked()
            );
            assert_eq!(
                &'a',
                list.front_node()
                    .unwrap()
                    .next()
                    .unwrap()
                    .next()
                    .unwrap()
                    .data_unchecked()
            );

            assert_eq!(vec!['c', 'b', 'a'], to_vec(&list));
            assert_eq!(vec!['a', 'b', 'c'], to_vec_from_back(&list));
        }

        test_with_all_memory_utilization(test);
    }

    #[test]
    fn push_back_front() {
        fn test(mut list: List<char>) {
            list.push_back('a');
            validate_full_slice(&list, &list.vec);

            list.push_front('b');
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some(&'a'), list.back());
            assert_eq!(
                &'b',
                list.back_node().unwrap().prev().unwrap().data_unchecked()
            );
            assert_eq!(Some(&'b'), list.front());
            assert_eq!(
                &'a',
                list.front_node().unwrap().next().unwrap().data_unchecked()
            );

            list.push_back('c');
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some(&'c'), list.back());
            assert_eq!(
                &'a',
                list.back_node().unwrap().prev().unwrap().data_unchecked()
            );
            assert_eq!(
                &'b',
                list.back_node()
                    .unwrap()
                    .prev()
                    .unwrap()
                    .prev()
                    .unwrap()
                    .data_unchecked()
            );
            assert_eq!(Some(&'b'), list.front());
            assert_eq!(
                &'a',
                list.front_node().unwrap().next().unwrap().data_unchecked()
            );
            assert_eq!(
                &'c',
                list.front_node()
                    .unwrap()
                    .next()
                    .unwrap()
                    .next()
                    .unwrap()
                    .data_unchecked()
            );

            assert_eq!(vec!['b', 'a', 'c'], to_vec(&list));
            assert_eq!(vec!['c', 'a', 'b'], to_vec_from_back(&list));
        }

        test_with_all_memory_utilization(test);
    }

    #[test]
    fn pop_back() {
        fn test(mut list: List<char>) {
            assert!(list.pop_back().is_none());

            list.push_back('a');
            list.push_back('b');
            list.push_back('c');
            validate_full_slice(&list, &list.vec);
            assert_eq!(
                vec!['a', 'b', 'c'],
                list.iter().copied().collect::<Vec<_>>()
            );

            let popped = list.pop_back();
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some('c'), popped);
            assert_eq!(Some(&'b'), list.back());
            assert_eq!(Some(&'a'), list.front());
            assert_eq!(vec!['a', 'b'], to_vec(&list));

            let popped = list.pop_back();
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some('b'), popped);
            assert_eq!(Some(&'a'), list.back());
            assert_eq!(Some(&'a'), list.front());
            assert_eq!(vec!['a'], to_vec(&list));

            let popped = list.pop_back();
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some('a'), popped);
            assert_eq!(0, list.iter().count());

            let popped = list.pop_back();
            validate_full_slice(&list, &list.vec);
            assert!(popped.is_none());
        }

        test_with_all_memory_utilization(test);
    }

    #[test]
    fn pop_front() {
        fn test(mut list: List<char>) {
            assert!(list.pop_front().is_none());

            list.push_back('a');
            list.push_back('b');
            list.push_back('c');
            validate_full_slice(&list, &list.vec);
            assert_eq!(
                vec!['a', 'b', 'c'],
                list.iter().copied().collect::<Vec<_>>()
            );

            let popped = list.pop_front();
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some('a'), popped);
            assert_eq!(Some(&'c'), list.back());
            assert_eq!(Some(&'b'), list.front());
            assert_eq!(vec!['b', 'c'], to_vec(&list));

            let popped = list.pop_front();
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some('b'), popped);
            assert_eq!(Some(&'c'), list.back());
            assert_eq!(Some(&'c'), list.front());
            assert_eq!(vec!['c'], to_vec(&list));

            let popped = list.pop_front();
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some('c'), popped);
            assert_eq!(0, list.iter().count());

            let popped = list.pop_front();
            validate_full_slice(&list, &list.vec);
            assert!(popped.is_none());
        }

        test_with_all_memory_utilization(test);
    }

    #[test]
    fn pop_back_front() {
        fn test(mut list: List<char>) {
            list.push_back('a');
            list.push_back('b');
            list.push_back('c');

            let popped = list.pop_back();
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some('c'), popped);
            assert_eq!(Some(&'b'), list.back());
            assert_eq!(Some(&'a'), list.front());
            assert_eq!(vec!['a', 'b'], to_vec(&list));

            let popped = list.pop_front();
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some('a'), popped);
            assert_eq!(Some(&'b'), list.back());
            assert_eq!(Some(&'b'), list.front());
            assert_eq!(vec!['b'], to_vec(&list));

            let popped = list.pop_back();
            validate_full_slice(&list, &list.vec);
            assert_eq!(Some('b'), popped);
            assert_eq!(0, list.iter().count());

            let popped = list.pop_front();
            validate_full_slice(&list, &list.vec);
            assert!(popped.is_none());
        }

        test_with_all_memory_utilization(test);
    }

    #[test]
    fn remove_at() {
        fn test(mut list: List<char>) {
            list.push_back('a');
            list.push_back('b');
            list.push_back('c');
            list.push_back('d');
            list.push_back('e');

            assert!(list.remove_at(list.len()).is_none());
            validate_full_slice(&list, &list.vec);

            assert_eq!(Some('b'), list.remove_at(1));
            assert!(list.remove_at(list.len()).is_none());
            validate_full_slice(&list, &list.vec);

            assert_eq!(Some('d'), list.remove_at(2));
            assert!(list.remove_at(list.len()).is_none());
            validate_full_slice(&list, &list.vec);

            assert_eq!(Some('a'), list.remove_at(0));
            assert!(list.remove_at(list.len()).is_none());
            validate_full_slice(&list, &list.vec);

            assert_eq!(Some('e'), list.remove_at(1));
            assert!(list.remove_at(list.len()).is_none());
            validate_full_slice(&list, &list.vec);

            assert_eq!(Some('c'), list.remove_at(0));
            assert!(list.remove_at(list.len()).is_none());
            validate_full_slice(&list, &list.vec);

            assert!(list.is_empty());
        }

        test_with_all_memory_utilization(test);
    }

    #[test]
    fn insert_at() {
        fn test(mut list: List<char>) {
            list.push_back('a');
            list.push_back('b');
            list.push_back('c');

            list.insert_at(0, 'x');
            assert_eq!(vec!['x', 'a', 'b', 'c'], to_vec(&list));
            validate_full_slice(&list, &list.vec);
            list.pop_front();

            list.insert_at(3, 'x');
            assert_eq!(vec!['a', 'b', 'c', 'x'], to_vec(&list));
            validate_full_slice(&list, &list.vec);
            list.pop_back();

            list.insert_at(1, 'x');
            assert_eq!(vec!['a', 'x', 'b', 'c'], to_vec(&list));
            validate_full_slice(&list, &list.vec);
            list.remove_at(1);

            list.insert_at(2, 'x');
            assert_eq!(vec!['a', 'b', 'x', 'c'], to_vec(&list));
            validate_full_slice(&list, &list.vec);
            list.remove_at(2);

            assert_eq!(vec!['a', 'b', 'c'], to_vec(&list));
        }

        test_with_all_memory_utilization(test);
    }

    #[test]
    #[should_panic]
    fn insert_at_out_of_bounds() {
        fn test(mut list: List<char>) {
            list.push_back('a');
            list.push_back('b');
            list.push_back('c');

            list.insert_at(4, 'x');
        }
        test_with_all_memory_utilization(test);
    }

    #[test]
    fn memory_status() {
        #![allow(clippy::float_cmp)]

        let mut list = List::new().with_memory_utilization(MemoryUtilization::Lazy);

        list.push_back('a');
        list.push_back('b');
        list.push_back('c');

        assert_eq!(list.memory_status(), MemoryStatus::of_list(3, 3));
        assert_eq!(list.memory_status().utilization(), 1.0);

        list.pop_back();
        assert_eq!(list.memory_status(), MemoryStatus::of_list(2, 3));
        assert_eq!(list.memory_status().utilization(), 2.0 / 3.0);

        list.insert_at(1, 'c');
        assert_eq!(list.memory_status(), MemoryStatus::of_list(3, 4));
        assert_eq!(list.memory_status().utilization(), 3.0 / 4.0);

        list.pop_front();
        assert_eq!(list.memory_status(), MemoryStatus::of_list(2, 4));
        assert_eq!(list.memory_status().utilization(), 2.0 / 4.0);

        list.reclaim_memory();
        assert_eq!(list.memory_status(), MemoryStatus::of_list(2, 2));
        assert_eq!(list.memory_status().utilization(), 2.0 / 2.0);
    }

    #[test]
    fn as_slice() {
        let mut list = LinkedList::new();
        list.push_back('a');
        list.push_back('b');

        assert_eq!(list, &['a', 'b']);
        assert_eq!(list.as_slice(), &['a', 'b']);
    }
}
