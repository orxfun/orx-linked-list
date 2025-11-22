use crate::{
    DoublyIdx, List,
    type_aliases::{BACK_IDX, FRONT_IDX, IDX_ERR},
    variant::Doubly,
};
use orx_selfref_col::{MemoryPolicy, NodeIdx, NodeIdxError};

impl<T, M> List<Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
{
    // mut

    /// ***O(1)*** Removes and returns value at the given `idx` of the list.
    ///
    /// # Panics
    ///
    /// Panics:
    /// * if the `idx` is invalid (`idx_err` is not None for the index),
    /// * if the element with the given `idx` is already removed from the list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    /// list.push_back('c');
    /// list.push_back('d');
    /// let idx = list.push_front('b');
    /// list.push_front('a');
    /// list.push_back('e');
    ///
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd', 'e']));
    ///
    /// let value = list.remove(idx);
    ///
    /// assert_eq!(value, 'b');
    /// assert!(list.eq_to_iter_vals(['a', 'c', 'd', 'e']));
    /// ```
    pub fn remove(&mut self, idx: DoublyIdx<T>) -> T {
        let idx = self.0.try_get_ptr(idx).expect(IDX_ERR);
        let [prev, next] = {
            let node = self.0.node(&idx);
            [node.prev().get().cloned(), node.next().get().cloned()]
        };

        match prev.clone() {
            Some(prev) => self.0.node_mut(&prev).next_mut().set(next.clone()),
            None => self.0.ends_mut().set(FRONT_IDX, next.clone()),
        }

        match next {
            Some(next) => self.0.node_mut(&next).prev_mut().set(prev),
            None => self.0.ends_mut().set(BACK_IDX, prev),
        }

        self.0.close_and_reclaim(&idx)
    }

    /// ***O(1)*** Inserts the given `value` as the next of the node with the given `idx`.
    ///
    /// # Panics
    ///
    /// Panics:
    /// * if the `idx` is invalid (`idx_err` is not None for the index),
    /// * if the element with the given `idx` is already removed from the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// list.push_back('a');
    /// let b = list.push_back('b');
    /// list.push_back('c');
    /// list.push_back('d');
    ///
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd']));
    ///
    /// let x = list.insert_next_to(b, 'x');
    ///
    /// assert_eq!(list.get(x), Some(&'x'));
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'x', 'c', 'd']));
    ///```
    pub fn insert_next_to(&mut self, idx: DoublyIdx<T>, value: T) -> DoublyIdx<T> {
        let prev = self.0.try_get_ptr(idx).expect(IDX_ERR);
        let next = self.0.node(&prev).next().get().cloned();
        let idx = self.0.push(value);

        self.0.node_mut(&prev).next_mut().set_some(idx.clone());
        self.0.node_mut(&idx).prev_mut().set_some(prev);

        match next {
            Some(next) => {
                self.0.node_mut(&next).prev_mut().set_some(idx.clone());
                self.0.node_mut(&idx).next_mut().set_some(next);
            }
            None => self.0.ends_mut().set_some(BACK_IDX, idx.clone()),
        }

        NodeIdx::new(self.memory_state(), &idx)
    }

    /// ***O(1)*** Inserts the given `value` as the next of the node with the given `idx`.
    /// Returns the index of the inserted node.
    ///
    /// # Panics
    ///
    /// Panics:
    /// * if the `idx` is invalid (`idx_err` is not None for the index),
    /// * if the element with the given `idx` is already removed from the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// list.push_back('a');
    /// list.push_back('b');
    /// let c = list.push_back('c');
    /// list.push_back('d');
    ///
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd']));
    ///
    /// let x = list.insert_prev_to(c, 'x');
    ///
    /// assert_eq!(list.get(x), Some(&'x'));
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'x', 'c', 'd']));
    ///```
    pub fn insert_prev_to(&mut self, idx: DoublyIdx<T>, value: T) -> DoublyIdx<T> {
        let next = self.0.try_get_ptr(idx).expect(IDX_ERR);
        let prev = self.0.node(&next).prev().get().cloned();
        let idx = self.0.push(value);

        self.0.node_mut(&next).prev_mut().set_some(idx.clone());
        self.0.node_mut(&idx).next_mut().set_some(next);

        match prev {
            Some(prev) => {
                self.0.node_mut(&prev).next_mut().set_some(idx.clone());
                self.0.node_mut(&idx).prev_mut().set_some(prev);
            }
            None => self.0.ends_mut().set_some(FRONT_IDX, idx.clone()),
        }

        NodeIdx::new(self.memory_state(), &idx)
    }

    /// ***O(1)*** Removes and returns value at the given `idx` of the list.
    ///
    /// Does not change the list and returns None:
    /// * if the `idx` is invalid (`idx_err` is not None for the index),
    /// * if the element with the given `idx` is already removed from the list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    /// list.push_back('c');
    /// list.push_back('d');
    /// let idx = list.push_front('b');
    /// list.push_front('a');
    /// list.push_back('e');
    ///
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd', 'e']));
    ///
    /// let value = list.try_remove(idx);
    ///
    /// assert_eq!(value, Some('b'));
    /// assert!(list.eq_to_iter_vals(['a', 'c', 'd', 'e']));
    /// assert_eq!(list.idx_err(idx), Some(NodeIdxError::RemovedNode));
    ///
    /// let value = list.try_remove(idx);
    /// assert_eq!(value, None);
    /// ```
    pub fn try_remove(&mut self, idx: DoublyIdx<T>) -> Option<T> {
        let can_remove = self.0.node_mut_from_idx(idx).is_some_and(|n| n.is_active());
        match can_remove {
            true => {
                let idx = idx.node_ptr();
                let [prev, next] = {
                    let node = self.0.node(&idx);
                    [node.prev().get().cloned(), node.next().get().cloned()]
                };

                match prev.clone() {
                    Some(prev) => self.0.node_mut(&prev).next_mut().set(next.clone()),
                    None => self.0.ends_mut().set(FRONT_IDX, next.clone()),
                }

                match next {
                    Some(next) => self.0.node_mut(&next).prev_mut().set(prev),
                    None => self.0.ends_mut().set(BACK_IDX, prev),
                }

                Some(self.0.close_and_reclaim(&idx))
            }
            false => None,
        }
    }

    /// ***O(1)*** Inserts the given `value` as the next of the node with the given `idx`.
    /// Returns the index of the inserted node.
    ///
    /// Does not change the list and returns None:
    /// * if the `idx` is invalid (`idx_err` is not None for the index),
    /// * if the element with the given `idx` is already removed from the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// list.push_back('a');
    /// let b = list.push_back('b');
    /// list.push_back('c');
    /// list.push_back('d');
    ///
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd']));
    ///
    /// let x = list.try_insert_next_to(b, 'x').unwrap();
    ///
    /// assert_eq!(list.get(x), Some(&'x'));
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'x', 'c', 'd']));
    ///
    /// let _ = list.remove(b);
    /// assert!(list.eq_to_iter_vals(['a', 'x', 'c', 'd']));
    ///
    /// let y = list.try_insert_next_to(b, 'y');
    /// assert_eq!(y, Err(NodeIdxError::RemovedNode));
    /// assert!(list.eq_to_iter_vals(['a', 'x', 'c', 'd'])); // unchanged
    ///```
    pub fn try_insert_next_to(
        &mut self,
        idx: DoublyIdx<T>,
        value: T,
    ) -> Result<DoublyIdx<T>, NodeIdxError> {
        let prev = self.0.try_get_ptr(idx)?;
        let next = self.0.node(&prev).next().get().cloned();
        let idx = self.0.push(value);

        self.0.node_mut(&prev).next_mut().set_some(idx.clone());
        self.0.node_mut(&idx).prev_mut().set_some(prev);

        match next {
            Some(next) => {
                self.0.node_mut(&next).prev_mut().set_some(idx.clone());
                self.0.node_mut(&idx).next_mut().set_some(next);
            }
            None => self.0.ends_mut().set_some(BACK_IDX, idx.clone()),
        }

        Ok(NodeIdx::new(self.memory_state(), &idx))
    }

    /// ***O(1)*** Inserts the given `value` as the next of the node with the given `idx`.
    /// Returns the index of the inserted node.
    ///
    /// Does not change the list and returns None:
    /// * if the `idx` is invalid (`idx_err` is not None for the index),
    /// * if the element with the given `idx` is already removed from the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// list.push_back('a');
    /// list.push_back('b');
    /// let c = list.push_back('c');
    /// list.push_back('d');
    ///
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd']));
    ///
    /// let x = list.try_insert_prev_to(c, 'x').unwrap();
    ///
    /// assert_eq!(list.get(x), Some(&'x'));
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'x', 'c', 'd']));
    ///
    /// let _ = list.remove(c);
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'x', 'd']));
    ///
    /// let y = list.try_insert_prev_to(c, 'y');
    /// assert_eq!(y, Err(NodeIdxError::RemovedNode));
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'x', 'd'])); // unchanged
    ///```
    pub fn try_insert_prev_to(
        &mut self,
        idx: DoublyIdx<T>,
        value: T,
    ) -> Result<DoublyIdx<T>, NodeIdxError> {
        let next = self.0.try_get_ptr(idx)?;
        let prev = self.0.node(&next).prev().get().cloned();
        let idx = self.0.push(value);

        self.0.node_mut(&next).prev_mut().set_some(idx.clone());
        self.0.node_mut(&idx).next_mut().set_some(next);

        match prev {
            Some(prev) => {
                self.0.node_mut(&prev).next_mut().set_some(idx.clone());
                self.0.node_mut(&idx).prev_mut().set_some(prev);
            }
            None => self.0.ends_mut().set_some(FRONT_IDX, idx.clone()),
        }

        Ok(NodeIdx::new(self.memory_state(), &idx))
    }
}
