use crate::{type_aliases::OOB, Doubly, DoublyIterable, List, Singly, SinglyIterable};
use orx_selfref_col::{MemoryPolicy, NodeIdx};

// singly

impl<T, M> List<Singly<T>, M>
where
    M: MemoryPolicy<Singly<T>>,
{
    /// ***O(n)*** Inserts the given `value` at the `position`-th element of the list.
    ///
    /// Similar to push methods, returns an index to the inserted node to allow constant time access.
    ///
    /// Time complexity:
    /// * starts from the `front`,
    /// * ***O(n)*** iterates until reaching the element,
    /// * ***O(1)*** inserts the value.
    ///
    /// # Panics
    ///
    /// Panics if `position > self.len()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = SinglyList::from_iter(['b', 'c', 'd']);
    ///
    /// list.insert_at(0, 'a');
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd']));
    ///
    /// list.insert_at(4, 'e');
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd', 'e']));
    ///
    /// list.insert_at(3, 'x');
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'x', 'd', 'e']));
    /// ```
    pub fn insert_at(&mut self, position: usize, value: T) -> NodeIdx<Singly<T>> {
        match position {
            0 => self.push_front(value),
            x => {
                let prev = self.iter_ptr().nth(x - 1).expect(OOB);
                let idx = self.0.push(value);

                if let Some(next) = self.0.node(&prev).next().get() {
                    self.0.node_mut(&idx).next_mut().set_some(&next);
                }

                self.0.node_mut(&prev).next_mut().set_some(&idx);

                NodeIdx::new(self.memory_state(), &idx)
            }
        }
    }

    /// ***O(n)*** Removes and returns value at the `position`-th element of the list.
    /// Returns None if `position` is out-of-bounds.
    ///
    /// Time complexity:
    /// * starts from the `front`,
    /// * ***O(n)*** iterates until reaching the element,
    /// * ***O(1)*** removes the value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = SinglyList::from_iter(['a', 'b', 'c', 'd', 'e']);
    ///
    /// let value = list.remove_at(0);
    /// assert_eq!(value, Some('a'));
    /// assert!(list.eq_to_iter_vals(['b', 'c', 'd', 'e']));
    ///
    /// let value = list.remove_at(3);
    /// assert_eq!(value, Some('e'));
    /// assert!(list.eq_to_iter_vals(['b', 'c', 'd']));
    ///
    /// let value = list.remove_at(1);
    /// assert_eq!(value, Some('c'));
    /// assert!(list.eq_to_iter_vals(['b', 'd']));
    /// ```
    #[allow(clippy::missing_panics_doc, clippy::unwrap_in_result)]
    pub fn remove_at(&mut self, position: usize) -> Option<T> {
        match position {
            x if x >= self.len() => None,
            0 => self.pop_front(),
            x => {
                let (prev, idx) = {
                    let mut iter = self.iter_ptr();
                    let prev = iter.by_ref().nth(x - 1).expect(OOB);
                    let idx = iter.next().expect(OOB);
                    (prev, idx)
                };

                match self.0.node(&idx).next().get() {
                    Some(next) => self.0.node_mut(&prev).next_mut().set_some(&next),
                    None => self.0.node_mut(&prev).next_mut().set_none(),
                }

                Some(self.0.close_and_reclaim(&idx))
            }
        }
    }
}

// doubly

impl<T, M> List<Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
{
    /// ***O(n)*** Inserts the given `value` at the `position`-th element of the list.
    ///
    /// Time complexity:
    /// * starts from the `front`,
    /// * ***O(n)*** iterates until reaching the element,
    /// * ***O(1)*** inserts the value.
    ///
    /// # Panics
    ///
    /// Panics if `position > self.len()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::from_iter(['b', 'c', 'd']);
    ///
    /// list.insert_at(0, 'a');
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd']));
    ///
    /// list.insert_at(4, 'e');
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd', 'e']));
    ///
    /// list.insert_at(3, 'x');
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'x', 'd', 'e']));
    /// ```
    pub fn insert_at(&mut self, position: usize, value: T) -> NodeIdx<Doubly<T>> {
        match position {
            0 => self.push_front(value),
            x if x == self.len() => self.push_back(value),
            _ => {
                let prev = self.iter_ptr().nth(position - 1).expect(OOB);
                let idx = self.0.push(value);
                let next = self.0.node(&prev).next().get().expect("must exist");

                self.0.node_mut(&prev).next_mut().set_some(&idx);
                self.0.node_mut(&next).prev_mut().set_some(&idx);

                self.0.node_mut(&idx).prev_mut().set_some(&prev);
                self.0.node_mut(&idx).next_mut().set_some(&next);

                NodeIdx::new(self.memory_state(), &idx)
            }
        }
    }

    /// ***O(n)*** Inserts the given `value` at the `position_from_back`-th element of the list.
    ///
    /// Time complexity:
    /// * starts from the `front`,
    /// * ***O(n)*** iterates until reaching the element,
    /// * ***O(1)*** inserts the value.
    ///
    /// # Panics
    ///
    /// Panics if `position_from_back > self.len()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::from_iter(['b', 'c', 'd']);
    ///
    /// list.insert_at_from_back(0, 'e');
    /// assert!(list.eq_to_iter_vals(['b', 'c', 'd', 'e']));
    ///
    /// list.insert_at_from_back(4, 'a');
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd', 'e']));
    ///
    /// list.insert_at_from_back(2, 'x');
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'x', 'd', 'e']));
    /// ```
    pub fn insert_at_from_back(
        &mut self,
        position_from_back: usize,
        value: T,
    ) -> NodeIdx<Doubly<T>> {
        match position_from_back {
            0 => self.push_back(value),
            x if x == self.len() => self.push_front(value),
            x => {
                let mut iter = self.iter_ptr().rev();
                let next = iter.nth(x - 1).expect(OOB);
                let idx = self.0.push(value);
                let prev = self.0.node(&next).prev().get().expect("must exist");

                self.0.node_mut(&next).prev_mut().set_some(&idx);
                self.0.node_mut(&prev).next_mut().set_some(&idx);

                self.0.node_mut(&idx).prev_mut().set_some(&prev);
                self.0.node_mut(&idx).next_mut().set_some(&next);

                NodeIdx::new(self.memory_state(), &idx)
            }
        }
    }

    /// ***O(n)*** Removes and returns value at the `position`-th element of the list.
    /// Returns None if `position` is out-of-bounds.
    ///
    /// Time complexity:
    /// * starts from the `front`,
    /// * ***O(n)*** iterates until reaching the element,
    /// * ***O(1)*** removes the value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::from_iter(['a', 'b', 'c', 'd', 'e']);
    ///
    /// let value = list.remove_at(0);
    /// assert_eq!(value, Some('a'));
    /// assert!(list.eq_to_iter_vals(['b', 'c', 'd', 'e']));
    ///
    /// let value = list.remove_at(3);
    /// assert_eq!(value, Some('e'));
    /// assert!(list.eq_to_iter_vals(['b', 'c', 'd']));
    ///
    /// let value = list.remove_at(1);
    /// assert_eq!(value, Some('c'));
    /// assert!(list.eq_to_iter_vals(['b', 'd']));
    /// ```
    #[allow(clippy::missing_panics_doc, clippy::unwrap_in_result)]
    pub fn remove_at(&mut self, position: usize) -> Option<T> {
        match position {
            x if x >= self.len() => None,
            0 => self.pop_front(),
            x if x == self.len() - 1 => self.pop_back(),
            x => {
                let (prev, idx, next) = {
                    let mut iter = self.iter_ptr();
                    let prev = iter.by_ref().nth(x - 1).expect(OOB);
                    let idx = iter.next().expect(OOB);
                    let next = iter.next().expect(OOB);
                    (prev, idx, next)
                };

                self.0.node_mut(&prev).next_mut().set_some(&next);
                self.0.node_mut(&next).prev_mut().set_some(&prev);

                Some(self.0.close_and_reclaim(&idx))
            }
        }
    }

    /// ***O(n)*** Removes and returns value at the `position_from_back`-th element from the back of the list.
    /// Returns None if `position` is out-of-bounds.
    ///
    /// Time complexity:
    /// * starts from the `back`,
    /// * ***O(n)*** iterates until reaching the element,
    /// * ***O(1)*** removes the value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::from_iter(['a', 'b', 'c', 'd', 'e']);
    ///
    /// let value = list.remove_at_from_back(4);
    /// assert_eq!(value, Some('a'));
    /// assert!(list.eq_to_iter_vals(['b', 'c', 'd', 'e']));
    ///
    /// let value = list.remove_at_from_back(0);
    /// assert_eq!(value, Some('e'));
    /// assert!(list.eq_to_iter_vals(['b', 'c', 'd']));
    ///
    /// let value = list.remove_at_from_back(1);
    /// assert_eq!(value, Some('c'));
    /// assert!(list.eq_to_iter_vals(['b', 'd']));
    /// ```
    #[allow(clippy::missing_panics_doc, clippy::unwrap_in_result)]
    pub fn remove_at_from_back(&mut self, position_from_back: usize) -> Option<T> {
        match position_from_back {
            x if x >= self.len() => None,
            0 => self.pop_back(),
            x if x == self.len() - 1 => self.pop_front(),
            x => {
                let (prev, idx, next) = {
                    let mut iter = self.iter_ptr().rev();
                    let next = iter.by_ref().nth(x - 1).expect(OOB);
                    let idx = iter.next().expect(OOB);
                    let prev = iter.next().expect(OOB);
                    (prev, idx, next)
                };

                self.0.node_mut(&prev).next_mut().set_some(&next);
                self.0.node_mut(&next).prev_mut().set_some(&prev);

                Some(self.0.close_and_reclaim(&idx))
            }
        }
    }
}
