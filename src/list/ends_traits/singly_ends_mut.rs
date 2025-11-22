use super::SinglyEnds;
use crate::{Singly, SinglyIdx, list::helper_traits::HasSinglyEndsMut};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node, NodeIdxError};

/// A list or view having a single end: front.
pub trait SinglyEndsMut<T, M, P>: HasSinglyEndsMut<T, M, P> + SinglyEnds<T, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
    /// ***O(1)*** Returns a mutable reference to the front of the list,
    /// returns None if the list is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = SinglyList::new();
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
            .get()
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
    fn get_mut<'a>(&'a mut self, idx: SinglyIdx<T>) -> Option<&'a mut T>
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
    fn try_get_mut<'a>(&'a mut self, idx: SinglyIdx<T>) -> Result<&'a mut T, NodeIdxError>
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
    /// list.push_front('d');
    /// list.push_front('c');
    /// let b = list.push_front('b');
    /// list.push_front('a');
    ///
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd']));
    ///
    /// let c = list.next_mut_of(b);
    /// *c.unwrap() = 'x';
    ///
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'x', 'd']));
    /// ```
    fn next_mut_of<'a>(&'a mut self, idx: SinglyIdx<T>) -> Option<&'a mut T>
    where
        M: 'a,
        P: 'a,
    {
        self.next_idx_of(idx).and_then(|i| self.get_mut(i))
    }
}

impl<L, T, M, P> SinglyEndsMut<T, M, P> for L
where
    L: HasSinglyEndsMut<T, M, P>,
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
{
}
