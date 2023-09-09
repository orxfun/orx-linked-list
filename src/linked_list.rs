use crate::{linked_list_x::LinkedListX, mem::MemoryUtilization, node::LinkedListNode};
use orx_imp_vec::prelude::*;

/// The LinkedList allows pushing and popping elements at either end in constant time.
///
/// Also see [`LinkedListX`] for the **structurally immutable** version of the linked list.
#[derive(Default)]
pub struct LinkedList<'a, T, P = SplitVec<LinkedListNode<'a, T>>>
where
    T: 'a,
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
{
    pub(crate) vec: ImpVec<LinkedListNode<'a, T>, P>,
    pub(crate) len: usize,
    /// Memory utilization strategy of the linked list allowing control over the tradeoff between
    /// memory efficiency and amortized time complexity of `pop_back`, `pop_front` or `remove` operations.
    ///
    /// See [MemoryUtilization] for details.
    pub memory_utilization: MemoryUtilization,
}

impl<'a, T, P> LinkedList<'a, T, P>
where
    T: 'a,
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
{
    /// Converts the `LinkedList` to a `LinkedList` stopping all **structural mutations**.
    ///
    /// Mutations of element data depends on whether the created `LinkedListX` is assigned
    /// to an immutable variable or `mut` variable.
    ///
    /// See the documentation of [`LinkedListX`] for details.
    pub fn built(self) -> LinkedListX<'a, T, P> {
        LinkedListX {
            vec: self.vec.into_pinned(),
            len: self.len,
            marker: Default::default(),
        }
    }
    /// Covnerts the linked list into one with the given `memory_utilization`.
    pub fn with_memory_utilization(self, memory_utilization: MemoryUtilization) -> Self {
        Self {
            vec: self.vec,
            len: self.len,
            memory_utilization: memory_utilization.into_valid(),
        }
    }

    /// Returns the length of the linked list.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
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
    /// use orx_linked_list::prelude::*;
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
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_doubling_growth(4);
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
        self.back_node().and_then(|node| node.data.as_ref())
    }
    /// Provides a mutable reference to the back element, or None if the list is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(16);
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
        self.node_ind(self.back_node())
            .and_then(|ind| self.vec[ind].data.as_mut())
    }
    /// Provides a reference to the front element, or None if the list is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_doubling_growth(4);
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
        self.front_node().and_then(|node| node.data.as_ref())
    }
    /// Provides a mutable reference to the front element, or None if the list is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(16);
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
        self.node_ind(self.front_node())
            .and_then(|ind| self.vec[ind].data.as_mut())
    }

    /// Appends an element to the back of a list.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_exponential_growth(2, 1.5);
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
        match self.back_node_ind() {
            None => self.push_first_node(value),
            Some(prior_back_ind) => {
                let ind = Some(self.vec.len());
                self.vec.push(LinkedListNode {
                    data: Some(value),
                    next: None,
                    prev: self.back_node(),
                });
                self.vec.set_next(prior_back_ind, ind);
                self.set_back(ind);
            }
        }
        self.len += 1;
    }
    /// Appends an element to the back of a list.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_exponential_growth(2, 1.5);
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
        match self.front_node_ind() {
            None => self.push_first_node(value),
            Some(prior_front_ind) => {
                let ind = Some(self.vec.len());
                self.vec.push(LinkedListNode {
                    data: Some(value),
                    next: self.front_node(),
                    prev: None,
                });
                self.vec.set_prev(prior_front_ind, ind);
                self.set_front(ind);
            }
        }
        self.len += 1;
    }

    /// Removes the last element from a list and returns it, or None if it is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_exponential_growth(2, 1.5);
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
        let value = if let Some(old_back_ind) = self.back_node_ind() {
            let new_back_ind = self.node_ind(self.vec[old_back_ind].prev);
            self.set_back(new_back_ind);

            if let Some(new_back_ind) = new_back_ind {
                self.vec.set_next(new_back_ind, None);
            } else {
                self.set_front(None);
            }

            self.len -= 1;
            Some(self.remove_get_at(old_back_ind))
        } else {
            None
        };
        self.reclaim_memory_if_necessary(value.is_some());
        value
    }

    /// Removes the last element from a list and returns it, or None if it is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_exponential_growth(2, 1.5);
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
        let value = if let Some(old_front_ind) = self.front_node_ind() {
            let new_front_ind = self.node_ind(self.vec[old_front_ind].next);
            self.set_front(new_front_ind);

            if let Some(new_front_ind) = new_front_ind {
                self.vec.set_prev(new_front_ind, None);
            } else {
                self.set_back(None);
            }

            self.len -= 1;
            Some(self.remove_get_at(old_front_ind))
        } else {
            None
        };
        self.reclaim_memory_if_necessary(value.is_some());
        value
    }

    /// Removes all elements from the LinkedList.
    ///
    /// This operation should compute in O(n) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_exponential_growth(2, 1.5);
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
        self.vec.push(LinkedListNode::back_front_node());
        self.len = 0;
    }

    /// Removes the element at the given index and returns it.
    ///
    /// This operation requires *O*(*n*) time to access the `at`-th element and constant time to remove.
    ///
    /// # Panics
    /// Panics if at >= len
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(8);
    ///
    /// // build linked list: x <-> a <-> b <-> c
    /// list.push_back('a');
    /// list.push_back('b');
    /// list.push_front('x');
    /// list.push_back('c');
    ///
    /// assert_eq!(list.remove_at(1), 'a');
    /// assert_eq!(list.remove_at(0), 'x');
    /// assert_eq!(list.remove_at(1), 'c');
    /// assert_eq!(list.remove_at(0), 'b');
    /// ```
    pub fn remove_at(&mut self, at: usize) -> T {
        let curr = self.node_at(at);

        // update vec ends
        if at == 0 {
            // prev | front
            self.vec[0].prev = curr.next;
        }
        if at == self.len - 1 {
            // next | back
            self.vec[0].next = curr.prev;
        }

        // update links
        if let Some(prev_ind) = self.node_ind(curr.prev) {
            self.vec.set_next(prev_ind, self.node_ind(curr.next));
        }
        if let Some(next_ind) = self.node_ind(curr.next) {
            self.vec.set_prev(next_ind, self.node_ind(curr.prev));
        }

        let imp_at = self.node_ind(Some(curr)).expect(IS_SOME);
        let value = self.remove_get_at(imp_at);
        self.reclaim_memory_if_necessary(true);
        self.len -= 1;
        value
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
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(8);
    ///
    /// // build linked list: a <-> b <-> c
    /// list.push_back('a');
    /// list.push_back('b');
    /// list.push_back('c');
    ///
    /// list.insert_at(1, 'w');
    /// assert_eq!(vec!['a', 'w', 'b', 'c'], list.collect_vec());
    /// ```
    pub fn insert_at(&mut self, at: usize, value: T) {
        if at == self.len {
            self.push_back(value);
        } else {
            let curr = self.node_at(at);
            let curr_ind = self.node_ind(Some(curr)).expect(IS_SOME);
            let curr_prev_ind = self.node_ind(curr.prev);

            let ind = self.vec.len();
            let node = self.vec.push_get_ref(LinkedListNode {
                data: Some(value),
                prev: curr.prev,
                next: Some(curr),
            });

            // update links
            self.vec.set_prev(curr_ind, Some(ind));
            if let Some(prev_ind) = curr_prev_ind {
                self.vec.set_next(prev_ind, Some(ind));
            }

            // update vec ends
            if at == 0 {
                // prev | front
                self.vec[0].prev = Some(node);
            }

            self.len += 1;
        }
    }

    /// Returns a reference to element at the `at` position starting from the `front`;
    /// None when `at` is out of bounds.
    ///
    /// This operation requires *O*(*n*) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(8);
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
        if at < self.len {
            self.node_at(at).data.as_ref()
        } else {
            None
        }
    }
    /// Returns a mutable reference to element at the `at` position starting from the `front`;
    /// None when `at` is out of bounds.
    ///
    /// This operation requires *O*(*n*) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(8);
    ///
    /// // build linked list: a <-> b <-> c
    /// list.push_back('b');
    /// list.push_front('a');
    /// list.push_back('c');
    ///
    /// *list.get_mut_at(0).unwrap() = 'x';
    /// *list.get_mut_at(1).unwrap() = 'y';
    /// *list.get_mut_at(2).unwrap() = 'z';
    /// assert_eq!(None, list.get_mut_at(3));
    ///
    /// assert_eq!(Some(&'x'), list.get_at(0));
    /// assert_eq!(Some(&'y'), list.get_at(1));
    /// assert_eq!(Some(&'z'), list.get_at(2));
    /// ```
    pub fn get_mut_at(&mut self, at: usize) -> Option<&mut T> {
        if at < self.len {
            let node = self.node_at(at);
            let ind = self.node_ind(Some(node))?;
            self.vec[ind].data.as_mut()
        } else {
            None
        }
    }

    // helpers
    fn reclaim_memory_if_necessary(&mut self, condition_to_reclaim: bool)
    where
        P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    {
        if condition_to_reclaim {
            match &self.memory_utilization {
                MemoryUtilization::Eager => _ = self.memory_reclaim(),
                MemoryUtilization::Lazy => {}
                MemoryUtilization::WithThreshold(threshold) => {
                    if self.len > 0 {
                        let utilization = self.len as f32 / (self.vec.len() - 1) as f32;
                        if utilization < *threshold {
                            _ = self.memory_reclaim()
                        }
                    }
                }
            }
        }
    }
    /// Returns index of the referenced node:
    ///
    /// * might return None only if `node.is_none()`;
    /// * when `node.is_some()` it is expected to be a valid reference;
    /// hence, the method panics if not.
    ///
    /// # Safety
    ///
    /// Since this method, as well as the `LinkedListNode` struct are internal
    /// to this crate; it is never expected to receive an argument where the
    /// Some variant of the reference does not belong to the underlying imp
    /// vector.
    /// Therefore, `expect` call in the method body will never panic.
    pub(crate) fn node_ind(&self, node: Option<&'a LinkedListNode<'a, T>>) -> Option<usize> {
        node.map(|node_ref| self.vec.index_of(node_ref).expect(IS_SOME))
    }
    #[inline(always)]
    pub(crate) fn back_node(&self) -> Option<&'a LinkedListNode<'a, T>> {
        self.vec[0].next
    }
    #[inline(always)]
    pub(crate) fn front_node(&self) -> Option<&'a LinkedListNode<'a, T>> {
        self.vec[0].prev
    }
    #[inline(always)]
    pub(crate) fn back_node_ind(&self) -> Option<usize> {
        self.node_ind(self.back_node())
    }
    #[inline(always)]
    pub(crate) fn front_node_ind(&self) -> Option<usize> {
        self.node_ind(self.front_node())
    }
    #[inline(always)]
    pub(crate) fn set_back(&mut self, back_idx: Option<usize>) {
        self.vec.set_next(0, back_idx);
    }
    #[inline(always)]
    pub(crate) fn set_front(&mut self, front_idx: Option<usize>) {
        self.vec.set_prev(0, front_idx);
    }
    fn push_first_node(&mut self, value: T) {
        debug_assert!(self.vec[0].prev.is_none());
        debug_assert!(self.vec[0].next.is_none());
        let ind = Some(self.vec.len());
        self.vec.push(LinkedListNode {
            data: Some(value),
            prev: None,
            next: None,
        });
        self.vec.set_prev(0, ind);
        self.vec.set_next(0, ind);
    }
    fn node_at(&self, at: usize) -> &'a LinkedListNode<'a, T> {
        self.panic_if_out_of_bounds(at);
        let mut curr = self.vec[0].prev.expect(IS_SOME);
        for _ in 0..at {
            curr = curr.next.expect(IS_SOME);
        }
        curr
    }
    fn remove_get_at(&mut self, imp_at: usize) -> T {
        std::mem::replace(&mut self.vec[imp_at], LinkedListNode::closed_node())
            .data
            .expect(IS_SOME)
    }
    fn panic_if_out_of_bounds(&self, idx: usize) {
        assert!(
            idx < self.len,
            "Cannot remove at an index outside of the list bounds"
        );
    }
}

pub(crate) const IS_SOME: &str = "the data of an active node must be Some variant";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LinkedListDoubling;

    #[test]
    fn len_is_empty() {
        let mut list = LinkedList::with_doubling_growth(4);

        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        list.push_back(1);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);

        list.push_front(2);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 2);

        list.pop_back();
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);

        list.pop_front();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        list.push_back(1);
        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn back_front() {
        let mut list = LinkedList::with_linear_growth(16);

        assert_eq!(None, list.back());
        assert_eq!(None, list.front());
        assert_eq!(None, list.pop_back());
        assert_eq!(None, list.pop_front());

        list.push_back("hello");
        assert_eq!(Some(&"hello"), list.back());
        assert_eq!(Some(&"hello"), list.front());
        assert_eq!(Some("hello"), list.pop_back());
        assert_eq!(None, list.pop_front());

        list.push_front("world");
        assert_eq!(Some(&"world"), list.back());
        assert_eq!(Some(&"world"), list.front());
        assert_eq!(Some("world"), list.pop_front());
        assert_eq!(None, list.pop_back());

        list.push_back("hello");
        list.push_back("world");
        assert_eq!(Some(&"world"), list.back());
        assert_eq!(Some(&"hello"), list.front());

        list.push_back("!");
        assert_eq!(Some(&"!"), list.back());

        assert_eq!(Some("hello"), list.pop_front());
        assert_eq!(Some("!"), list.pop_back());
        assert_eq!(Some("world"), list.pop_front());
    }

    #[test]
    fn back_front_mut() {
        let mut list = LinkedList::<usize, _>::with_exponential_growth(8, 1.25);

        assert_eq!(None, list.back_mut());
        assert_eq!(None, list.front_mut());

        // 10 - 20 - 30 - 40
        list.push_back(20);
        list.push_back(30);
        list.push_front(10);
        list.push_back(40);

        let back = list.back_mut();
        assert_eq!(Some(&mut 40), back);
        *back.expect("is-some") *= 10;
        assert_eq!(Some(&400), list.back());
        assert_eq!(Some(400), list.pop_back());

        let front = list.front_mut();
        assert_eq!(Some(&mut 10), front);
        *front.expect("is-some") *= 10;
        assert_eq!(Some(&100), list.front());
        assert_eq!(Some(100), list.pop_front());
    }

    #[test]
    fn remove() {
        let mut list = LinkedList::with_doubling_growth(2);

        list.push_back(3);
        list.push_front(2);
        list.push_front(1);
        list.push_back(4);
        list.push_front(0);

        assert_eq!(vec![0, 1, 2, 3, 4], list.collect_vec());

        let removed = list.remove_at(4);
        assert_eq!(4, removed);
        assert_eq!(vec![0, 1, 2, 3], list.collect_vec());

        let removed = list.remove_at(2);
        assert_eq!(2, removed);
        assert_eq!(vec![0, 1, 3], list.collect_vec());

        let removed = list.remove_at(0);
        assert_eq!(0, removed);
        assert_eq!(vec![1, 3], list.collect_vec());

        let removed = list.remove_at(1);
        assert_eq!(3, removed);
        assert_eq!(vec![1], list.collect_vec());

        let removed = list.remove_at(0);
        assert_eq!(1, removed);
        assert!(list.collect_vec().is_empty());
    }

    #[test]
    #[should_panic]
    fn remove_out_of_bounds() {
        let mut list = LinkedList::with_doubling_growth(2);

        list.push_back(3);
        list.push_front(2);
        list.push_front(1);
        list.push_back(4);
        list.push_front(0);

        assert_eq!(vec![0, 1, 2, 3, 4], list.collect_vec());

        _ = list.remove_at(5);
    }

    #[test]
    fn insert() {
        fn get_list<'a>() -> LinkedListDoubling<'a, usize> {
            let mut list = LinkedList::with_doubling_growth(2);

            list.push_back(3);
            list.push_front(2);
            list.push_front(1);
            list.push_back(4);
            list.push_front(0);

            assert_eq!(vec![0, 1, 2, 3, 4], list.collect_vec());
            list
        }

        for i in 0..=5 {
            let mut expected = vec![0, 1, 2, 3, 4];
            expected.insert(i, 42);

            let mut list = get_list();
            list.insert_at(i, 42);

            assert_eq!(expected, list.collect_vec());
        }
    }

    #[test]
    fn get_at() {
        let mut list = LinkedList::with_doubling_growth(4);

        for i in 0..1000 {
            list.push_back(i);
        }

        for i in 0..list.len() {
            assert_eq!(Some(&i), list.get_at(i));
        }
        assert_eq!(None, list.get_at(1000));

        for i in 0..list.len() {
            *list.get_mut_at(i).expect(IS_SOME) *= 10;
        }
        assert_eq!(None, list.get_mut_at(1000));

        for i in 0..list.len() {
            assert_eq!(Some(&(i * 10)), list.get_at(i));
        }
        assert_eq!(None, list.get_at(1000));
    }
}
