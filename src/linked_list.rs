use crate::node::LinkedListNode;
use orx_imp_vec::prelude::{ImpVec, PinnedVec, SplitVec};

#[derive(Default)]
pub struct LinkedList<'a, T, P = SplitVec<LinkedListNode<'a, T>>>
where
    P: PinnedVec<LinkedListNode<'a, T>>,
{
    pub(crate) imp: ImpVec<LinkedListNode<'a, T>, P>,
    pub(crate) len: usize,
}

impl<'a, T, P> LinkedList<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
{
    /// Returns the length of the LinkedList.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_exponential_growth(2, 1.5);
    /// assert_eq!(0, list.len());
    ///
    /// list.push_front('a');
    /// list.push_front('b');
    /// assert_eq!(2, list.len());
    ///
    /// _ = list.pop_back();
    /// assert_eq!(1, list.len());
    ///
    /// _ = list.pop_front();
    /// assert_eq!(0, list.len());
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
    /// let mut list = LinkedList::with_exponential_growth(2, 1.5);
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
            .and_then(|ind| self.imp[ind].data.as_mut())
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
            .and_then(|ind| self.imp[ind].data.as_mut())
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
                let ind = Some(self.imp.len());
                self.imp.push(LinkedListNode {
                    data: Some(value),
                    next: None,
                    prev: self.back_node(),
                });
                self.imp.set_next(prior_back_ind, ind);
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
                let ind = Some(self.imp.len());
                self.imp.push(LinkedListNode {
                    data: Some(value),
                    next: self.front_node(),
                    prev: None,
                });
                self.imp.set_prev(prior_front_ind, ind);
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
        if let Some(old_back_ind) = self.back_node_ind() {
            let new_back_ind = self.node_ind(self.imp[old_back_ind].prev);
            self.set_back(new_back_ind);

            if let Some(new_back_ind) = new_back_ind {
                self.imp.set_next(new_back_ind, None);
            } else {
                self.set_front(None);
            }

            self.len -= 1;
            Some(self.remove_get_at(old_back_ind))
        } else {
            None
        }
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
        if let Some(old_front_ind) = self.front_node_ind() {
            let new_front_ind = self.node_ind(self.imp[old_front_ind].next);
            self.set_front(new_front_ind);

            if let Some(new_front_ind) = new_front_ind {
                self.imp.set_prev(new_front_ind, None);
            } else {
                self.set_back(None);
            }

            self.len -= 1;
            Some(self.remove_get_at(old_front_ind))
        } else {
            None
        }
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
        self.imp.clear();
        self.imp.push(LinkedListNode::back_front_node());
        self.len = 0;
    }

    /// Removes the element at the given index and returns it.
    ///
    /// This operation should compute in *O*(*n*) time
    /// to access the `at`-th element and constant time to remove.
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
    /// assert_eq!(list.remove(1), 'a');
    /// assert_eq!(list.remove(0), 'x');
    /// assert_eq!(list.remove(1), 'c');
    /// assert_eq!(list.remove(0), 'b');
    /// ```
    pub fn remove(&mut self, at: usize) -> T {
        let len = self.len();
        assert!(
            at < len,
            "Cannot remove at an index outside of the list bounds"
        );

        let mut curr = self.imp[0].prev.expect(IS_SOME);
        for _ in 0..at {
            curr = curr.next.expect(IS_SOME);
        }
        let imp_at = self.node_ind(Some(curr)).expect(IS_SOME);

        // update vec ends
        if at == 0 {
            // prev | front
            self.imp[0].prev = curr.next;
        }
        if at == len - 1 {
            // next | back
            self.imp[0].next = curr.prev;
        }

        // update links
        if let Some(prev_ind) = self.node_ind(curr.prev) {
            self.imp.set_next(prev_ind, self.node_ind(curr.next));
        }
        if let Some(next_ind) = self.node_ind(curr.next) {
            self.imp.set_prev(next_ind, self.node_ind(curr.prev));
        }

        self.len -= 1;
        self.remove_get_at(imp_at)
    }

    // helpers
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
        node.map(|node_ref| self.imp.index_of(node_ref).expect(IS_SOME))
    }
    #[inline(always)]
    pub(crate) fn back_node(&self) -> Option<&'a LinkedListNode<'a, T>> {
        self.imp[0].next
    }
    #[inline(always)]
    pub(crate) fn front_node(&self) -> Option<&'a LinkedListNode<'a, T>> {
        self.imp[0].prev
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
        self.imp.set_next(0, back_idx);
    }
    #[inline(always)]
    pub(crate) fn set_front(&mut self, front_idx: Option<usize>) {
        self.imp.set_prev(0, front_idx);
    }
    fn push_first_node(&mut self, value: T) {
        debug_assert!(self.imp[0].prev.is_none());
        debug_assert!(self.imp[0].next.is_none());
        let ind = Some(self.imp.len());
        self.imp.push(LinkedListNode {
            data: Some(value),
            prev: None,
            next: None,
        });
        self.imp.set_prev(0, ind);
        self.imp.set_next(0, ind);
    }
    fn remove_get_at(&mut self, imp_at: usize) -> T {
        std::mem::replace(&mut self.imp[imp_at], LinkedListNode::closed_node())
            .data
            .expect(IS_SOME)
    }
}

const IS_SOME: &str = "the data of an active node must be Some variant";
