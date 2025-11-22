use crate::{
    Doubly, DoublyIdx,
    list::helper_traits::HasDoublyEnds,
    type_aliases::{BACK_IDX, FRONT_IDX, IDX_ERR},
};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node, NodeIdxError};

/// A list or view having two ends: front and back.
pub trait DoublyEnds<T, M, P>: HasDoublyEnds<T, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    /// ***O(1)*** Returns a reference to the front of the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// assert!(list.front().is_none());
    ///
    /// list.push_front('a');
    /// assert_eq!(Some(&'a'), list.front());
    ///
    /// list.push_front('b');
    /// assert_eq!(Some(&'b'), list.front());
    ///
    /// _ = list.pop_front();
    /// assert_eq!(Some(&'a'), list.front());
    /// ```
    fn front<'a>(&'a self) -> Option<&'a T>
    where
        M: 'a,
        P: 'a,
    {
        self.ends()
            .get(FRONT_IDX)
            .map(|p| unsafe { self.col().data_unchecked(p) })
    }

    /// ***O(1)*** Returns a reference to the back of the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// assert!(list.back().is_none());
    ///
    /// list.push_back('a');
    /// assert_eq!(Some(&'a'), list.back());
    ///
    /// list.push_back('b');
    /// assert_eq!(Some(&'b'), list.back());
    ///
    /// list.push_front('c');
    /// assert_eq!(Some(&'b'), list.back());
    ///
    /// _ = list.pop_back();
    /// assert_eq!(Some(&'a'), list.back());
    /// ```
    fn back<'a>(&'a self) -> Option<&'a T>
    where
        M: 'a,
        P: 'a,
    {
        self.ends()
            .get(BACK_IDX)
            .map(|p| unsafe { self.col().data_unchecked(p) })
    }

    // idx

    /// ***O(1)*** Returns a None if the given node `idx` is valid.
    ///
    /// Returns Some of the corresponding NodeIdxError if the index is invalid.
    ///
    /// # Safety
    ///
    /// Returns `None` if all of the following safety conditions hold:
    /// * the index is created from this list,
    /// * the node that this index is created for still belongs to the list (not removed),
    /// * the node positions in this list are not reorganized to reclaim memory:
    ///   * DoublyList or SinglyList automatically reorganizes nodes on removal of items
    ///     if the utilization of memory drops below a threshold.
    ///   * DoublyListLazy or SinglyListLazy do not reorganize nodes implicitly,
    ///     the indices are only invalidated if the `reclaim_closed_nodes` is manually called.
    ///
    /// Returns the error otherwise.
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
    /// assert_eq!(list.idx_err(a), None);
    /// assert_eq!(list.idx_err(b), None);
    ///
    /// list.push_front('c');
    /// list.push_back('d');
    /// list.push_front('e');
    /// let f = list.push_back('f');
    ///
    /// assert_eq!(list.idx_err(a), None);
    /// assert_eq!(list.idx_err(b), None);
    /// assert_eq!(list.idx_err(f), None);
    ///
    /// let _ = list.pop_back(); // f is removed
    ///
    /// assert_eq!(list.idx_err(a), None);
    /// assert_eq!(list.idx_err(b), None);
    /// assert_eq!(list.idx_err(f), Some(NodeIdxError::RemovedNode));
    ///
    /// list.clear(); // all removed
    ///
    /// assert_eq!(list.idx_err(a), Some(NodeIdxError::OutOfBounds));
    /// assert_eq!(list.idx_err(b), Some(NodeIdxError::OutOfBounds));
    /// assert_eq!(list.idx_err(f), Some(NodeIdxError::OutOfBounds));
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
    /// assert_eq!(list.idx_err(c), None);
    ///
    /// list.pop_back(); // does not lead to reorganization
    ///
    /// assert_eq!(list.idx_err(c), None);
    ///
    /// list.pop_front(); // leads to reorganization
    ///
    /// assert_eq!(list.idx_err(c), Some(NodeIdxError::ReorganizedCollection));
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
    /// assert_eq!(list.idx_err(idx), None);
    /// // assert_eq!(list.idx_err(other_idx), Some(NodeIdxError::OutOfBounds));
    /// ```
    fn idx_err(&self, idx: DoublyIdx<T>) -> Option<NodeIdxError> {
        self.col().try_get_ptr(idx).err()
    }

    /// ***O(1)*** Returns whether or not the `idx` is valid for this list; i.e.,
    /// * the element corresponding to the `idx` is not removed from the list, and
    /// * the list is not reorganized after `idx` was created.
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
    /// assert_eq!(list.is_valid(a), true);
    /// assert_eq!(list.is_valid(b), true);
    ///
    /// list.push_front('c');
    /// list.push_back('d');
    /// list.push_front('e');
    /// let f = list.push_back('f');
    ///
    /// assert_eq!(list.is_valid(a), true);
    /// assert_eq!(list.is_valid(b), true);
    /// assert_eq!(list.is_valid(f), true);
    ///
    /// let _ = list.pop_back(); // f is removed
    ///
    /// assert_eq!(list.is_valid(a), true);
    /// assert_eq!(list.is_valid(b), true);
    /// assert_eq!(list.is_valid(f), false);
    ///
    /// list.clear(); // all removed
    ///
    /// assert_eq!(list.is_valid(a), false);
    /// assert_eq!(list.is_valid(b), false);
    /// assert_eq!(list.is_valid(f), false);
    /// ```
    ///
    /// In the following, removal of nodes invalidates indices due to reorganization.
    /// In these cases, `is_valid` returns false.
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
    /// assert_eq!(list.is_valid(c), true);
    ///
    /// list.pop_back(); // does not lead to reorganization
    ///
    /// assert_eq!(list.is_valid(c), true);
    ///
    /// list.pop_front(); // leads to reorganization
    ///
    /// assert_eq!(list.is_valid(c), false);
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
    /// assert_eq!(list.is_valid(idx), true);
    /// // assert_eq!(list.is_valid(other_idx), false);
    /// ```
    fn is_valid(&self, idx: DoublyIdx<T>) -> bool {
        self.col().try_get_ptr(idx).is_ok()
    }

    /// ***O(1)*** Returns a reference to the node with the given `idx` in constant time.
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
    /// We can use `try_get` to understand why the index is invalid.
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
    /// list.push_front('c');
    /// list.push_back('d');
    /// list.push_front('e');
    /// let f = list.push_back('f');
    ///
    /// assert_eq!(list.get(a), Some(&'a'));
    /// assert_eq!(list.get(b), Some(&'b'));
    /// assert_eq!(list.get(f), Some(&'f'));
    ///
    /// let _ = list.pop_back(); // f is removed
    ///
    /// assert_eq!(list.get(a), Some(&'a'));
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
    /// assert_eq!(list.get(c), Some(&'c'));
    ///
    /// list.pop_back(); // does not lead to reorganization
    ///
    /// assert_eq!(list.get(c), Some(&'c'));
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
    /// assert_eq!(list.get(idx), Some(&'a'));
    /// // assert_eq!(list.get(other_idx), None);
    /// ```
    fn get<'a>(&'a self, idx: DoublyIdx<T>) -> Option<&'a T>
    where
        M: 'a,
        P: 'a,
    {
        self.col().node_from_idx(idx).and_then(|n| n.data())
    }

    /// ***O(1)*** Returns a reference to the node with the given `idx` in constant time.
    ///
    /// Returns NodeIdxError if the index is invalid.
    ///
    /// # Safety
    ///
    /// Returns `Ok` if all of the following safety conditions hold:
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
    /// list.push_front('c');
    /// list.push_back('d');
    /// list.push_front('e');
    /// let f = list.push_back('f');
    ///
    /// assert_eq!(list.try_get(a), Ok(&'a'));
    /// assert_eq!(list.try_get(b), Ok(&'b'));
    /// assert_eq!(list.try_get(f), Ok(&'f'));
    ///
    /// let _ = list.pop_back(); // f is removed
    ///
    /// assert_eq!(list.try_get(a), Ok(&'a'));
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
    /// In these cases, we safely receive an error.
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
    /// assert_eq!(list.try_get(c), Ok(&'c'));
    ///
    /// list.pop_back(); // does not lead to reorganization
    ///
    /// assert_eq!(list.try_get(c), Ok(&'c'));
    ///
    /// list.pop_front(); // leads to reorganization
    ///
    /// assert_eq!(list.try_get(c), Err(NodeIdxError::ReorganizedCollection));
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
    /// assert_eq!(list.try_get(idx), Ok(&'a'));
    /// // assert_eq!(list.try_get(other_idx), Err(NodeIdxError::OutOfBounds));
    /// ```
    fn try_get<'a>(&'a self, idx: DoublyIdx<T>) -> Result<&'a T, NodeIdxError>
    where
        M: 'a,
        P: 'a,
    {
        self.col()
            .try_node_from_idx(idx)
            .and_then(|x| match x.data() {
                Some(x) => Ok(x),
                None => Err(NodeIdxError::RemovedNode),
            })
    }

    /// ***O(1)*** Returns the index of the element succeeding the one with the given `idx`.
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
    /// let c = list.next_idx_of(a).and_then(|b| list.next_idx_of(b)).unwrap();
    /// let d = list.next_idx_of(c).unwrap();
    ///
    /// assert_eq!(list.get(c), Some(&'c'));
    /// assert_eq!(list.get(d), Some(&'d'));
    ///
    /// assert!(list.next_idx_of(d).is_none());
    /// ```
    fn next_idx_of(&self, idx: DoublyIdx<T>) -> Option<DoublyIdx<T>> {
        let ptr = self.col().try_get_ptr(idx).expect(IDX_ERR);
        let next_ptr = self.col().node(ptr).next().get();
        next_ptr.map(|p| DoublyIdx::new(self.col().memory_state(), p))
    }

    /// ***O(1)*** Returns the element succeeding the one with the given `idx`.
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
    /// let c = list.next_idx_of(a).and_then(|b| list.next_of(b));
    /// assert_eq!(c, Some(&'c'));
    /// ```
    fn next_of<'a>(&'a self, idx: DoublyIdx<T>) -> Option<&'a T>
    where
        M: 'a,
        P: 'a,
    {
        self.next_idx_of(idx).and_then(|i| self.get(i))
    }

    /// ***O(1)*** Returns the index of the element preceding the one with the given `idx`.
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
    /// list.push_back('c');
    /// list.push_front('b');
    /// list.push_front('a');
    /// let d = list.push_back('d');
    ///
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd']));
    ///
    /// let b = list.prev_idx_of(d).and_then(|c| list.prev_idx_of(c)).unwrap();
    /// let a = list.prev_idx_of(b).unwrap();
    ///
    /// assert_eq!(list.get(b), Some(&'b'));
    /// assert_eq!(list.get(a), Some(&'a'));
    ///
    /// assert!(list.prev_idx_of(a).is_none());
    /// ```
    fn prev_idx_of(&self, idx: DoublyIdx<T>) -> Option<DoublyIdx<T>> {
        let ptr = self.col().try_get_ptr(idx).expect(IDX_ERR);
        let prev_ptr = self.col().node(ptr).prev().get();
        prev_ptr.map(|p| DoublyIdx::new(self.col().memory_state(), p))
    }

    /// ***O(1)*** Returns the element preceding the one with the given `idx`.
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
    /// let a = list.prev_idx_of(c).and_then(|b| list.prev_of(b));
    /// assert_eq!(a, Some(&'a'));
    /// ```
    fn prev_of<'a>(&'a self, idx: DoublyIdx<T>) -> Option<&'a T>
    where
        M: 'a,
        P: 'a,
    {
        self.prev_idx_of(idx).and_then(|i| self.get(i))
    }
}

impl<L, T, M, P> DoublyEnds<T, M, P> for L
where
    L: HasDoublyEnds<T, M, P>,
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
}
