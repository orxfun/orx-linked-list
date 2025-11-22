use crate::{
    Doubly, DoublyEnds, DoublyIdx,
    list::helper_traits::HasDoublyEndsMut,
    type_aliases::{BACK_IDX, FRONT_IDX, IDX_ERR, OOB},
};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node, NodeIdx, NodeIdxError};

/// A list or view having a single end: front.
pub trait DoublyEndsMut<T, M, P>: HasDoublyEndsMut<T, M, P> + DoublyEnds<T, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    /// ***O(1)*** Returns a mutable reference to the front of the list,
    /// returns None if the list is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// assert!(list.front_mut().is_none());
    ///
    /// list.push_front('a');
    /// assert_eq!(Some(&'a'), list.front());
    ///
    /// *list.front_mut().unwrap() = 'x';
    ///
    /// assert_eq!(Some(&'x'), list.front());
    /// ```
    fn front_mut<'a>(&'a mut self) -> Option<&'a mut T>
    where
        M: 'a,
        P: 'a,
    {
        self.ends_mut()
            .get(FRONT_IDX)
            .cloned()
            .map(|p| unsafe { self.col_mut().data_mut_unchecked(&p) })
    }

    /// ***O(1)*** Returns a mutable reference to the back of the list,
    /// returns None if the list is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// assert!(list.back_mut().is_none());
    ///
    /// list.push_back('a');
    /// assert_eq!(Some(&'a'), list.back());
    ///
    /// *list.back_mut().unwrap() = 'x';
    ///
    /// assert_eq!(Some(&'x'), list.back());
    /// ```
    fn back_mut<'a>(&'a mut self) -> Option<&'a mut T>
    where
        M: 'a,
        P: 'a,
    {
        self.ends_mut()
            .get(BACK_IDX)
            .cloned()
            .map(|p| unsafe { self.col_mut().data_mut_unchecked(&p) })
    }

    // idx

    /// ***O(1)*** Returns a mutable reference to the node with the given `idx` in constant time.
    ///
    /// Returns None if the index is invalid.
    ///
    /// # Safety
    ///
    /// Returns `Some` if all of the following safety conditions hold:
    /// * the index is created from this list,
    /// * the node that this index is created for still belongs to the list (not removed),
    /// * the node positions in this list are not reorganized to reclaim memory:
    ///   * DoublyList or SinglyList automatically reorganizes nodes on removal of items
    ///     if the utilization of memory drops below a threshold.
    ///   * DoublyListLazy or SinglyListLazy do not reorganize nodes implicitly,
    ///     the indices are only invalidated if the `reclaim_closed_nodes` is manually called.
    ///
    /// Returns `None` otherwise.
    /// We can use `try_get_mut` to understand why the index is invalid.
    ///
    /// # Examples
    ///
    /// Following example illustrates where automatic reorganization does not happen since
    /// no elements are removed from the list.
    ///
    /// The indices remain valid.
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// let a = list.push_back('a');
    /// let b = list.push_back('b');
    ///
    /// assert_eq!(list.get(a), Some(&'a'));
    /// assert_eq!(list.get(b), Some(&'b'));
    ///
    /// *list.get_mut(a).unwrap() = 'x';
    ///
    /// list.push_front('c');
    /// list.push_back('d');
    /// list.push_front('e');
    /// let f = list.push_back('f');
    ///
    /// assert_eq!(list.get(a), Some(&'x'));
    /// assert_eq!(list.get(b), Some(&'b'));
    /// assert_eq!(list.get(f), Some(&'f'));
    ///
    /// let _ = list.pop_back(); // f is removed
    ///
    /// *list.get_mut(a).unwrap() = 'y';
    ///
    /// assert_eq!(list.get(a), Some(&'y'));
    /// assert_eq!(list.get(b), Some(&'b'));
    /// assert_eq!(list.get(f), None);
    ///
    /// list.clear(); // all removed
    ///
    /// assert_eq!(list.get(a), None);
    /// assert_eq!(list.get(b), None);
    /// assert_eq!(list.get(f), None);
    /// ```
    ///
    /// In the following, removal of nodes invalidates indices due to reorganization.
    /// In these cases, we safely receive None.
    ///
    /// Note that, to have complete control on validity of indices, we can use
    /// DoublyListLazy or SinglyListLazy.
    /// In these variants, indices are invalidated only if we manually call `reclaim_closed_nodes`.
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
    /// list.push_back('e');
    ///
    /// *list.get_mut(c).unwrap() = 'x';
    ///
    /// list.pop_back(); // does not lead to reorganization
    ///
    /// assert_eq!(list.get(c), Some(&'x'));
    ///
    /// list.pop_front(); // leads to reorganization
    ///
    /// assert_eq!(list.get(c), None);
    /// ```
    ///
    /// In the final example, we attempt to access to a list element using an index created by another list.
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    /// let idx = list.push_back('a');
    ///
    /// let mut other_list = DoublyList::new();
    /// let other_idx = other_list.push_back('a');
    ///
    /// assert!(list.get_mut(idx).is_some());
    /// // assert_eq!(list.get_mut(other_idx), None);
    /// ```
    fn get_mut<'a>(&'a mut self, idx: DoublyIdx<T>) -> Option<&'a mut T>
    where
        M: 'a,
        P: 'a,
    {
        self.col_mut()
            .node_mut_from_idx(idx)
            .and_then(|n| n.data_mut())
    }

    /// ***O(1)*** Returns a mutable reference to the node with the given `idx` in constant time.
    ///
    /// Returns NodeIdxError if the index is invalid.
    ///
    /// # Safety
    ///
    /// Returns `Some` if all of the following safety conditions hold:
    /// * the index is created from this list,
    /// * the node that this index is created for still belongs to the list (not removed),
    /// * the node positions in this list are not reorganized to reclaim memory:
    ///   * DoublyList or SinglyList automatically reorganizes nodes on removal of items
    ///     if the utilization of memory drops below a threshold.
    ///   * DoublyListLazy or SinglyListLazy do not reorganize nodes implicitly,
    ///     the indices are only invalidated if the `reclaim_closed_nodes` is manually called.
    ///
    /// Otherwise, returns:
    /// * RemovedNode if the particular element is removed from the list.
    /// * OutOfBounds if the index is does not point to the current nodes of the list.
    /// * ReorganizedCollection if nodes of the list are reorganized to reclaim closed nodes.
    ///
    /// # Examples
    ///
    /// Following example illustrates where automatic reorganization does not happen since
    /// no elements are removed from the list.
    ///
    /// The indices remain valid.
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// let a = list.push_back('a');
    /// let b = list.push_back('b');
    ///
    /// assert_eq!(list.try_get(a), Ok(&'a'));
    /// assert_eq!(list.try_get(b), Ok(&'b'));
    ///
    /// *list.try_get_mut(a).unwrap() = 'x';
    ///
    /// list.push_front('c');
    /// list.push_back('d');
    /// list.push_front('e');
    /// let f = list.push_back('f');
    ///
    /// assert_eq!(list.try_get(a), Ok(&'x'));
    /// assert_eq!(list.try_get(b), Ok(&'b'));
    /// assert_eq!(list.try_get(f), Ok(&'f'));
    ///
    /// let _ = list.pop_back(); // f is removed
    ///
    /// *list.try_get_mut(a).unwrap() = 'y';
    ///
    /// assert_eq!(list.try_get(a), Ok(&'y'));
    /// assert_eq!(list.try_get(b), Ok(&'b'));
    /// assert_eq!(list.try_get(f), Err(NodeIdxError::RemovedNode));
    ///
    /// list.clear(); // all removed
    ///
    /// assert_eq!(list.try_get(a), Err(NodeIdxError::OutOfBounds));
    /// assert_eq!(list.try_get(b), Err(NodeIdxError::OutOfBounds));
    /// assert_eq!(list.try_get(f), Err(NodeIdxError::OutOfBounds));
    /// ```
    ///
    /// In the following, removal of nodes invalidates indices due to reorganization.
    /// In these cases, we safely receive None.
    ///
    /// Note that, to have complete control on validity of indices, we can use
    /// DoublyListLazy or SinglyListLazy.
    /// In these variants, indices are invalidated only if we manually call `reclaim_closed_nodes`.
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
    /// list.push_back('e');
    ///
    /// *list.try_get_mut(c).unwrap() = 'x';
    ///
    /// list.pop_back(); // does not lead to reorganization
    ///
    /// assert_eq!(list.get(c), Some(&'x'));
    ///
    /// list.pop_front(); // leads to reorganization
    ///
    /// assert_eq!(list.try_get_mut(c), Err(NodeIdxError::ReorganizedCollection));
    /// ```
    ///
    /// In the final example, we attempt to access to a list element using an index created by another list.
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    /// let idx = list.push_back('a');
    ///
    /// let mut other_list = DoublyList::new();
    /// let other_idx = other_list.push_back('a');
    ///
    /// assert!(list.try_get_mut(idx).is_ok());
    /// // assert_eq!(list.try_get_mut(other_idx), Err(NodeIdxError::OutOfBounds));
    /// ```
    fn try_get_mut<'a>(&'a mut self, idx: DoublyIdx<T>) -> Result<&'a mut T, NodeIdxError>
    where
        M: 'a,
        P: 'a,
    {
        self.col_mut()
            .try_node_mut_from_idx(idx)
            .and_then(|x| match x.data_mut() {
                Some(x) => Ok(x),
                None => Err(NodeIdxError::RemovedNode),
            })
    }

    /// ***O(1)*** Returns a mutable reference to the element succeeding the one with the given `idx`.
    /// Returns None if the element at `idx` is the `back`.
    ///
    /// # Panics
    ///
    /// Panics if the `idx` is not valid; i.e., `idx_err` is not None.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// list.push_back('c');
    /// list.push_front('b');
    /// let a = list.push_front('a');
    /// list.push_back('d');
    ///
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd']));
    ///
    /// let c = list.next_idx_of(a).and_then(|b| list.next_mut_of(b));
    /// *c.unwrap() = 'x';
    ///
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'x', 'd']));
    /// ```
    fn next_mut_of<'a>(&'a mut self, idx: DoublyIdx<T>) -> Option<&'a mut T>
    where
        M: 'a,
        P: 'a,
    {
        self.next_idx_of(idx).and_then(|i| self.get_mut(i))
    }

    /// ***O(1)*** Returns a mutable reference to the element preceding the one with the given `idx`.
    /// Returns None if the element at `idx` is the `front`.
    ///
    /// # Panics
    ///
    /// Panics if the `idx` is not valid; i.e., `idx_err` is not None.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// let c = list.push_back('c');
    /// list.push_front('b');
    /// list.push_front('a');
    /// list.push_back('d');
    ///
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd']));
    ///
    /// let a = list.prev_idx_of(c).and_then(|b| list.prev_mut_of(b));
    /// *a.unwrap() = 'x';
    ///
    /// assert!(list.eq_to_iter_vals(['x', 'b', 'c', 'd']));
    /// ```
    fn prev_mut_of<'a>(&'a mut self, idx: DoublyIdx<T>) -> Option<&'a mut T>
    where
        M: 'a,
        P: 'a,
    {
        self.prev_idx_of(idx).and_then(|i| self.get_mut(i))
    }

    /// ***O(n)*** Reverses the list (in-place).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// let c = list.push_back('c');
    /// let _b = list.push_front('b');
    /// let _a = list.push_front('a');
    /// let _d = list.push_back('d');
    /// let e = list.push_back('e');
    ///
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd', 'e']));
    ///
    /// list.reverse();
    /// assert!(list.eq_to_iter_vals(['e', 'd', 'c', 'b', 'a']));
    ///
    /// let mut slice = list.slice_mut(&e..=&c);
    /// assert!(slice.eq_to_iter_vals(['e', 'd', 'c']));
    ///
    /// slice.reverse();
    /// assert!(slice.eq_to_iter_vals(['c', 'd', 'e']));
    ///
    /// assert!(list.eq_to_iter_vals(['c', 'd', 'e', 'b', 'a']));
    /// ```
    fn reverse(&mut self) {
        if let Some(front) = self.ends().get(FRONT_IDX).cloned() {
            let back = self.ends().get(BACK_IDX).cloned().expect("exists");

            if front == back {
                return;
            }

            let new_next_of_front = self.col().node(&back).next().get().cloned();
            let new_prev_of_back = self.col().node(&front).prev().get().cloned();

            let mut prev = front.clone();
            let mut new_next = self.col().node(&prev).next().get().cloned();

            while let Some(next) = new_next {
                new_next = self.col().node(&next).next().get().cloned();

                self.link(&next, &prev);

                prev = next;
                if prev == back {
                    break;
                }
            }

            match new_next_of_front {
                Some(new_next_of_front) => self.link(&front, &new_next_of_front),
                None => self.col_mut().node_mut(&front).next_mut().set_none(),
            }

            match new_prev_of_back {
                Some(new_prev_of_back) => self.link(&new_prev_of_back, &back),
                None => self.col_mut().node_mut(&back).prev_mut().set_none(),
            }

            // ends

            let old_col_front = self.col().ends().get(FRONT_IDX).cloned().expect("exists");
            let old_col_back = self.col().ends().get(BACK_IDX).cloned().expect("exists");

            self.ends_mut().set_some(FRONT_IDX, back.clone());
            self.ends_mut().set_some(BACK_IDX, front.clone());

            if front == old_col_front {
                self.col_mut().ends_mut().set_some(FRONT_IDX, back.clone());
            }

            if back == old_col_back {
                self.col_mut().ends_mut().set_some(BACK_IDX, front);
            }
        }
    }

    // idx - move

    /// ***O(1)*** Moves the element with the given `idx`
    /// immediately after the target element with the given `idx_target`.
    ///
    /// # Panics
    ///
    /// Panics if either of the indices is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: DoublyList<_> = (0..6).collect();
    /// let idx: Vec<_> = list.indices().collect();
    ///
    /// assert!(list.eq_to_iter_vals([0, 1, 2, 3, 4, 5]));
    ///
    /// list.move_next_to(idx[4], idx[1]);
    /// assert!(list.eq_to_iter_vals([0, 1, 4, 2, 3, 5]));
    ///
    /// list.move_next_to(idx[2], idx[5]);
    /// assert!(list.eq_to_iter_vals([0, 1, 4, 3, 5, 2]));
    ///
    /// list.move_next_to(idx[3], idx[0]);
    /// assert!(list.eq_to_iter_vals([0, 3, 1, 4, 5, 2]));
    /// ```
    fn move_next_to(&mut self, idx: DoublyIdx<T>, idx_target: DoublyIdx<T>) {
        let mid = self.col().try_get_ptr(idx).expect(IDX_ERR);
        let prev = self.col().try_get_ptr(idx_target).expect(IDX_ERR);

        if mid == prev {
            return;
        }

        let next = self.col().node(&prev).next().get().cloned();
        let old_next = self.col().node(&mid).next().get().cloned();
        let old_prev = self.col().node(&mid).prev().get().cloned();

        // update the gap
        match (old_prev.clone(), old_next.clone()) {
            (Some(old_prev), _) if old_prev == prev => return,
            (Some(old_prev), Some(old_next)) => self.link(&old_prev, &old_next),
            (Some(old_prev), None) => {
                // idx must be col.back
                self.col_mut().node_mut(&old_prev).next_mut().set_none();

                self.col_mut().ends_mut().set_some(BACK_IDX, old_prev);
            }
            (None, Some(old_next)) => {
                // idx must be col.front
                self.col_mut().node_mut(&old_next).prev_mut().set_none();

                self.col_mut().ends_mut().set_some(FRONT_IDX, old_next);
            }
            (None, None) => return,
        }

        // update the fill
        match next {
            Some(next) => self.link(&mid, &next),
            None => self.col_mut().node_mut(&mid).next_mut().set_none(),
        }
        self.link(&prev, &mid);

        // custom ends
        let old_front = self.ends().get(FRONT_IDX).cloned();
        let old_back = self.ends().get(BACK_IDX).cloned();

        if let Some(old_back) = old_back.clone() {
            match old_back == prev {
                true => {
                    // new node placed in front
                    self.ends_mut().set_some(BACK_IDX, mid.clone())
                }
                false => {
                    if old_back == mid {
                        // old front is moved away
                        let old_front = old_front.clone().expect("exists");
                        match mid == old_front {
                            false => {
                                let new_back = old_prev.expect("exists");
                                self.ends_mut().set_some(BACK_IDX, new_back);
                            }
                            true => { /* singleton, no update */ }
                        }
                    }
                }
            }
        }

        if let Some(old_front) = old_front
            && old_front == mid
        {
            // old back is moved away
            let old_back = old_back.expect("exists");
            match old_front == old_back {
                false => {
                    let new_front = old_next.expect("exists");
                    self.ends_mut().set_some(FRONT_IDX, new_front);
                }
                true => { /* singleton, no update */ }
            }
        }
    }

    /// ***O(1)*** Moves the element with the given `idx`
    /// immediately before the target element with the given `idx_target`.
    ///
    /// # Panics
    ///
    /// Panics if either of the indices is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: DoublyList<_> = (0..6).collect();
    /// let idx: Vec<_> = list.indices().collect();
    ///
    /// assert!(list.eq_to_iter_vals([0, 1, 2, 3, 4, 5]));
    ///
    /// list.move_prev_to(idx[4], idx[1]);
    /// assert!(list.eq_to_iter_vals([0, 4, 1, 2, 3, 5]));
    ///
    /// list.move_prev_to(idx[2], idx[5]);
    /// assert!(list.eq_to_iter_vals([0, 4, 1, 3, 2, 5]));
    ///
    /// list.move_prev_to(idx[3], idx[0]);
    /// assert!(list.eq_to_iter_vals([3, 0, 4, 1, 2, 5]));
    /// ```
    fn move_prev_to(&mut self, idx: DoublyIdx<T>, idx_target: DoublyIdx<T>) {
        let mid = self.col().try_get_ptr(idx).expect(IDX_ERR);
        let next = self.col().try_get_ptr(idx_target).expect(IDX_ERR);

        if mid == next {
            return;
        }

        let prev = self.col().node(&next).prev().get().cloned();
        let old_next = self.col().node(&mid).next().get().cloned();
        let old_prev = self.col().node(&mid).prev().get().cloned();

        // update the gap
        match (old_prev.clone(), old_next.clone()) {
            (_, Some(old_next)) if old_next == next => return,
            (Some(old_prev), Some(old_next)) => self.link(&old_prev, &old_next),
            (Some(old_prev), None) => {
                // idx must be col.back
                self.col_mut().node_mut(&old_prev).next_mut().set_none();

                self.col_mut().ends_mut().set_some(BACK_IDX, old_prev);
            }
            (None, Some(old_next)) => {
                // idx must be col.front
                self.col_mut().node_mut(&old_next).prev_mut().set_none();

                self.col_mut().ends_mut().set_some(FRONT_IDX, old_next);
            }
            (None, None) => return,
        }

        // update the fill
        match prev {
            Some(prev) => self.link(&prev, &mid),
            None => self.col_mut().node_mut(&mid).prev_mut().set_none(),
        }
        self.link(&mid, &next);

        // custom ends
        let old_front = self.ends().get(FRONT_IDX).cloned();
        let old_back = self.ends().get(BACK_IDX).cloned();

        if let Some(old_front) = &old_front {
            match old_front == &next {
                true => {
                    // new node placed in front
                    self.ends_mut().set_some(FRONT_IDX, mid.clone())
                }
                false => {
                    if old_front == &mid {
                        // old front is moved away
                        let old_back = old_back.clone().expect("exists");
                        match mid == old_back {
                            false => {
                                let new_front = old_next.expect("exists");
                                self.ends_mut().set_some(FRONT_IDX, new_front);
                            }
                            true => { /* singleton, no update */ }
                        }
                    }
                }
            }
        }

        if let Some(old_back) = old_back
            && old_back == mid
        {
            // old back is moved away
            let old_front = old_front.expect("exists");
            match old_front == old_back {
                false => {
                    let new_back = old_prev.expect("exists");
                    self.ends_mut().set_some(BACK_IDX, new_back);
                }
                true => { /* singleton, no update */ }
            }
        }
    }

    /// ***O(1)*** Moves the element with the given `idx`
    /// to the front of the list.
    ///
    /// # Panics
    ///
    /// Panics if the index is invalid or if the list is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: DoublyList<_> = (0..6).collect();
    /// let idx: Vec<_> = list.indices().collect();
    ///
    /// assert!(list.eq_to_iter_vals([0, 1, 2, 3, 4, 5]));
    ///
    /// list.move_to_front(idx[5]);
    /// assert!(list.eq_to_iter_vals([5, 0, 1, 2, 3, 4]));
    ///
    /// list.move_to_front(idx[2]);
    /// assert!(list.eq_to_iter_vals([2, 5, 0, 1, 3, 4]));
    ///
    /// list.move_to_front(idx[3]);
    /// assert!(list.eq_to_iter_vals([3, 2, 5, 0, 1, 4]));
    /// ```
    fn move_to_front(&mut self, idx: DoublyIdx<T>) {
        let ptr = self.ends().get(FRONT_IDX).expect(OOB);
        let idx_target = NodeIdx::new(self.col().memory_state(), ptr);
        self.move_prev_to(idx, idx_target);
    }

    /// ***O(1)*** Moves the element with the given `idx`
    /// to the back of the list.
    ///
    /// # Panics
    ///
    /// Panics if the index is invalid or if the list is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: DoublyList<_> = (0..6).collect();
    /// let idx: Vec<_> = list.indices().collect();
    ///
    /// assert!(list.eq_to_iter_vals([0, 1, 2, 3, 4, 5]));
    ///
    /// list.move_to_back(idx[1]);
    /// assert!(list.eq_to_iter_vals([0, 2, 3, 4, 5, 1]));
    ///
    /// list.move_to_back(idx[4]);
    /// assert!(list.eq_to_iter_vals([0, 2, 3, 5, 1, 4]));
    ///
    /// list.move_to_back(idx[2]);
    /// assert!(list.eq_to_iter_vals([0, 3, 5, 1, 4, 2]));
    /// ```
    fn move_to_back(&mut self, idx: DoublyIdx<T>) {
        let ptr = self.ends().get(BACK_IDX).expect(OOB);
        let idx_target = NodeIdx::new(self.col().memory_state(), ptr);
        self.move_next_to(idx, idx_target);
    }

    /// ***O(1)*** Swaps the elements with indices `a` and `b`.
    ///
    /// # Panics
    ///
    /// Panics if the `idx` is not valid; i.e., `idx_err` is not None.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: DoublyList<_> = (0..6).collect();
    /// let idx: Vec<_> = list.indices().collect();
    ///
    /// assert!(list.eq_to_iter_vals([0, 1, 2, 3, 4, 5]));
    ///
    /// list.swap(idx[1], idx[5]);
    /// assert!(list.eq_to_iter_vals([0, 5, 2, 3, 4, 1]));
    ///
    /// list.swap(idx[4], idx[0]);
    /// assert!(list.eq_to_iter_vals([4, 5, 2, 3, 0, 1]));
    ///
    /// list.swap(idx[3], idx[5]);
    /// assert!(list.eq_to_iter_vals([4, 3, 2, 5, 0, 1]));
    /// ```
    fn swap(&mut self, idx_a: DoublyIdx<T>, idx_b: DoublyIdx<T>) {
        let a = self.col().try_get_ptr(idx_a).expect(IDX_ERR);
        let b = self.col().try_get_ptr(idx_b).expect(IDX_ERR);

        if a == b {
            return;
        }

        let p_a = self.col().node(&a).prev().get().cloned();
        let p_b = self.col().node(&b).prev().get().cloned();
        let n_a = self.col().node(&a).next().get().cloned();
        let n_b = self.col().node(&b).next().get().cloned();

        match (n_a.clone(), n_b.clone()) {
            (Some(n_a), _) if b == n_a => self.move_next_to(idx_a, idx_b),
            (_, Some(n_b)) if a == n_b => self.move_next_to(idx_b, idx_a),
            _ => {
                match p_a {
                    Some(p_a) => self.link(&p_a, &b),
                    None => self.col_mut().node_mut(&b).prev_mut().set_none(),
                }

                match p_b {
                    Some(p_b) => self.link(&p_b, &a),
                    None => self.col_mut().node_mut(&a).prev_mut().set_none(),
                }

                match n_a {
                    Some(n_a) => self.link(&b, &n_a),
                    None => self.col_mut().node_mut(&b).next_mut().set_none(),
                }

                match n_b {
                    Some(n_b) => self.link(&a, &n_b),
                    None => self.col_mut().node_mut(&a).next_mut().set_none(),
                }

                // cache custom ends
                let custom_front = match self.ends().get(FRONT_IDX).cloned() {
                    Some(x) if x == a => Some(b.clone()),
                    Some(x) if x == b => Some(a.clone()),
                    _ => None,
                };

                let custom_back = match self.ends().get(BACK_IDX).cloned() {
                    Some(x) if x == a => Some(b.clone()),
                    Some(x) if x == b => Some(a.clone()),
                    _ => None,
                };

                // update col ends
                match self.col().ends().get(FRONT_IDX).cloned() {
                    Some(x) if x == a => self.col_mut().ends_mut().set_some(FRONT_IDX, b.clone()),
                    Some(x) if x == b => self.col_mut().ends_mut().set_some(FRONT_IDX, a.clone()),
                    _ => {}
                }

                match self.col().ends().get(BACK_IDX).cloned() {
                    Some(x) if x == a => self.col_mut().ends_mut().set_some(BACK_IDX, b),
                    Some(x) if x == b => self.col_mut().ends_mut().set_some(BACK_IDX, a),
                    _ => {}
                }

                // update custom ends
                if let Some(new_front) = custom_front {
                    self.ends_mut().set_some(FRONT_IDX, new_front);
                }

                if let Some(new_back) = custom_back {
                    self.ends_mut().set_some(BACK_IDX, new_back);
                }
            }
        }
    }

    // unsafe api

    /// ***O(1)*** Adds a link between `a` and `b`; i.e.,
    /// * sets a as the prev of b,
    /// * sets b as the next of a.
    ///
    /// # Panics
    ///
    /// Panics if either of the indices `a` and `b` is not valid; i.e., `idx_err` is not None.
    ///
    /// # Safety
    ///
    /// This method is unsafe since it breaks the internal structure of the linked list when used alone.
    /// The caller must guarantee that the internal structure is maintained by a series of moves
    /// as illustrated in the example.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: DoublyList<_> = (0..8).collect();
    /// let idx: Vec<_> = list.indices().collect();
    ///
    /// assert!(list.eq_to_iter_vals([0, 1, 2, 3, 4, 5, 6, 7]));
    ///
    /// unsafe {
    ///     list.remove_link(idx[0], idx[1]);
    ///     list.remove_link(idx[2], idx[3]);
    ///     list.add_link(idx[0], idx[3]);
    ///     list.add_link(idx[7], idx[1]);
    ///     list.set_back(idx[2]);
    /// }
    ///
    /// assert!(list.eq_to_iter_vals([0, 3, 4, 5, 6, 7, 1, 2]));
    /// ```
    ///
    /// This example also makes it clear that the unsafe api is very useful;
    /// however, it must only be used through a safe method that defines a
    /// proved to be legal move as a combination of unsafe moves.
    unsafe fn add_link(&mut self, a: DoublyIdx<T>, b: DoublyIdx<T>) {
        let a = self.col().try_get_ptr(a).expect(OOB);
        let b = self.col().try_get_ptr(b).expect(OOB);
        self.link(&a, &b);
    }

    /// ***O(1)*** Removes a link between `a` and `b`; i.e.,
    /// * clears the prev of b,
    /// * clears the next of a.
    ///
    /// # Panics
    ///
    /// Panics if either of the indices `a` and `b` is not valid; i.e., `idx_err` is not None.
    /// Also panics in debug mode if the link between a and be does not exist.
    ///
    /// # Safety
    ///
    /// This method is unsafe since it breaks the internal structure of the linked list when used alone.
    /// The caller must guarantee that the internal structure is maintained by a series of moves
    /// as illustrated in the example.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: DoublyList<_> = (0..8).collect();
    /// let idx: Vec<_> = list.indices().collect();
    ///
    /// assert!(list.eq_to_iter_vals([0, 1, 2, 3, 4, 5, 6, 7]));
    ///
    /// unsafe {
    ///     list.remove_link(idx[0], idx[1]);
    ///     list.remove_link(idx[2], idx[3]);
    ///     list.add_link(idx[0], idx[3]);
    ///     list.add_link(idx[7], idx[1]);
    ///     list.set_back(idx[2]);
    /// }
    ///
    /// assert!(list.eq_to_iter_vals([0, 3, 4, 5, 6, 7, 1, 2]));
    /// ```
    ///
    /// This example also makes it clear that the unsafe api is very useful;
    /// however, it must only be used through a safe method that defines a
    /// proved to be legal move as a combination of unsafe moves.
    unsafe fn remove_link(&mut self, a: DoublyIdx<T>, b: DoublyIdx<T>) {
        let a = self.col().try_get_ptr(a).expect(OOB);
        let b = self.col().try_get_ptr(b).expect(OOB);
        self.unlink(&a, &b);
    }

    /// ***O(1)*** Sets the `front` of the list as the `new_front`.
    ///
    /// # Panics
    ///
    /// Panics if the index `new_front` is not valid; i.e., `idx_err` is not None.
    ///
    /// # Safety
    ///
    /// This method is unsafe since it breaks the internal structure of the linked list when used alone.
    /// The caller must guarantee that the internal structure is maintained by a series of moves
    /// as illustrated in the example.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: DoublyList<_> = (0..8).collect();
    /// let idx: Vec<_> = list.indices().collect();
    ///
    /// assert!(list.eq_to_iter_vals([0, 1, 2, 3, 4, 5, 6, 7]));
    ///
    /// unsafe {
    ///     list.remove_link(idx[0], idx[1]);
    ///     list.remove_link(idx[2], idx[3]);
    ///     list.add_link(idx[0], idx[3]);
    ///     list.add_link(idx[7], idx[1]);
    ///     list.set_back(idx[2]);
    /// }
    ///
    /// assert!(list.eq_to_iter_vals([0, 3, 4, 5, 6, 7, 1, 2]));
    /// ```
    ///
    /// This example also makes it clear that the unsafe api is very useful;
    /// however, it must only be used through a safe method that defines a
    /// proved to be legal move as a combination of unsafe moves.
    unsafe fn set_front(&mut self, new_front: DoublyIdx<T>) {
        let new_front = self.col().try_get_ptr(new_front).expect(OOB);
        self.col_mut().ends_mut().set_some(FRONT_IDX, new_front);
    }

    /// ***O(1)*** Sets the `back` of the list as the `new_back`.
    ///
    /// # Panics
    ///
    /// Panics if the index `new_back` is not valid; i.e., `idx_err` is not None.
    ///
    /// # Safety
    ///
    /// This method is unsafe since it breaks the internal structure of the linked list when used alone.
    /// The caller must guarantee that the internal structure is maintained by a series of moves
    /// as illustrated in the example.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: DoublyList<_> = (0..8).collect();
    /// let idx: Vec<_> = list.indices().collect();
    ///
    /// assert!(list.eq_to_iter_vals([0, 1, 2, 3, 4, 5, 6, 7]));
    ///
    /// unsafe {
    ///     list.remove_link(idx[0], idx[1]);
    ///     list.remove_link(idx[2], idx[3]);
    ///     list.add_link(idx[0], idx[3]);
    ///     list.add_link(idx[7], idx[1]);
    ///     list.set_back(idx[2]);
    /// }
    ///
    /// assert!(list.eq_to_iter_vals([0, 3, 4, 5, 6, 7, 1, 2]));
    /// ```
    ///
    /// This example also makes it clear that the unsafe api is very useful;
    /// however, it must only be used through a safe method that defines a
    /// proved to be legal move as a combination of unsafe moves.
    unsafe fn set_back(&mut self, new_back: DoublyIdx<T>) {
        let new_back = self.col().try_get_ptr(new_back).expect(OOB);
        self.col_mut().ends_mut().set_some(BACK_IDX, new_back);
    }
}

impl<L, T, M, P> DoublyEndsMut<T, M, P> for L
where
    L: HasDoublyEndsMut<T, M, P>,
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
}
