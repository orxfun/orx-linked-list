use crate::{
    Doubly, DoublyList, DoublyListLazy, DoublyListThreshold, List, Singly, SinglyList,
    SinglyListLazy, SinglyListThreshold, variant::ListVariant,
};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryReclaimNever, MemoryReclaimOnThreshold, MemoryReclaimer, Node};

impl<const D: usize, R, V, P> From<List<V, MemoryReclaimNever, P>>
    for List<V, MemoryReclaimOnThreshold<D, V, R>, P>
where
    V: ListVariant,
    R: MemoryReclaimer<V>,
    P: PinnedVec<Node<V>>,
{
    fn from(value: List<V, MemoryReclaimNever, P>) -> Self {
        Self(value.0.into())
    }
}

impl<const D: usize, R, V, P> From<List<V, MemoryReclaimOnThreshold<D, V, R>, P>>
    for List<V, MemoryReclaimNever, P>
where
    V: ListVariant,
    R: MemoryReclaimer<V>,
    P: PinnedVec<Node<V>>,
{
    fn from(value: List<V, MemoryReclaimOnThreshold<D, V, R>, P>) -> Self {
        Self(value.0.into())
    }
}

// transitions

impl<const D: usize, R, V, P> List<V, MemoryReclaimOnThreshold<D, V, R>, P>
where
    V: ListVariant,
    R: MemoryReclaimer<V>,
    P: PinnedVec<Node<V>>,
{
    /// Transforms the list into lazy memory reclaim mode, such as:
    /// * `DoublyList` is transformed into `DoublyListLazy`
    /// * `SinglyList` is transformed into `SinglyListLazy`
    ///
    /// This transformation has no cost, and can as well be reverted
    /// with no cost calling the [`into_auto_reclaim`] method.
    ///
    /// The lazy mode will never automatically reorganize nodes;
    /// and hence, will never invalidate node indices.
    ///
    /// It is still possible to manually call [`reclaim_closed_nodes`].
    ///
    /// [`reclaim_closed_nodes`]: crate::List::reclaim_closed_nodes
    /// [`into_auto_reclaim`]: crate::List::into_auto_reclaim
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: DoublyList<_> = DoublyList::new();
    ///
    /// // growing will never invalidate indices
    /// let a = list.push_back('a');
    /// let b = list.push_back('b');
    /// let c = list.push_front('c');
    /// let d = list.push_front('d');
    ///
    /// assert_eq!(list.node_utilization().num_active_nodes, 4);
    /// assert_eq!(list.node_utilization().num_closed_nodes, 0);
    ///
    /// // make lazy
    /// let mut list: DoublyListLazy<_> = list.into_lazy_reclaim();
    ///
    /// // now removals will never lead to an automatic reorganization
    /// // hence a, b, c, d will never be invalidated unless they are removed
    /// let pop = list.pop_back();
    /// assert_eq!(pop, Some('b'));
    ///
    /// assert_eq!(list.node_utilization().num_active_nodes, 3);
    /// assert_eq!(list.node_utilization().num_closed_nodes, 1);
    ///
    /// // we can still use the indices to have constant time access to nodes
    ///
    /// assert_eq!(list.idx_err(a), None);
    /// assert_eq!(list.idx_err(b), Some(NodeIdxError::RemovedNode));
    /// assert_eq!(list.idx_err(c), None);
    /// assert_eq!(list.idx_err(d), None);
    ///
    /// assert_eq!(list.get(a), Some(&'a'));
    /// assert_eq!(list.get(b), None);
    /// assert_eq!(list.get(c), Some(&'c'));
    /// assert_eq!(list.get(d), Some(&'d'));
    ///
    /// // make auto again
    /// let mut list: DoublyList<_> = list.into_auto_reclaim();
    ///
    /// // now removals might lead to reorganization if node utilization
    /// // falls below a certain threshold (75% when D=2).
    /// let pop = list.remove(d);
    /// assert_eq!(pop, 'd');
    ///
    /// // 2 removed nodes reclaimed (b, d); 2 active nodes remains (c, a)
    /// assert_eq!(list.node_utilization().num_active_nodes, 2);
    /// assert_eq!(list.node_utilization().num_closed_nodes, 0);
    ///
    /// // node positions might have change during reorganization
    /// assert_eq!(list.idx_err(a), Some(NodeIdxError::ReorganizedCollection));
    /// assert_eq!(list.idx_err(b), Some(NodeIdxError::ReorganizedCollection));
    /// // or prior position does not belong to the storage any more
    /// assert_eq!(list.idx_err(c), Some(NodeIdxError::OutOfBounds));
    /// assert_eq!(list.idx_err(d), Some(NodeIdxError::OutOfBounds));
    ///
    /// // indices can no longer be used to access the elements
    /// assert_eq!(list.get(a), None);
    /// assert_eq!(list.get(b), None);
    /// assert_eq!(list.get(c), None);
    /// assert_eq!(list.get(d), None);
    ///
    /// // we can recollect valid indices if necessary
    /// let idx: Vec<_> = list.indices().collect();
    /// assert_eq!(list.get(idx[0]), Some(&'c'));
    /// assert_eq!(list.get(idx[1]), Some(&'a'));
    /// ```
    pub fn into_lazy_reclaim(self) -> List<V, MemoryReclaimNever, P> {
        self.into()
    }
}

impl<T, P> DoublyListLazy<T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    /// Transforms the list into auto memory reclaim mode, such as:
    /// * `DoublyListLazy` is transformed into `DoublyList`
    /// * `SinglyListLazy` is transformed into `SinglyList`
    ///
    /// This transformation has no cost, and can as well be reverted
    /// with no cost calling the [`into_lazy_reclaim`] method.
    ///
    /// The auto mode will reclaim memory whenever the ratio of
    /// active nodes to total used nodes; i.e., node utilization,
    /// falls below a certain threshold.
    ///
    /// [`reclaim_closed_nodes`]: crate::List::reclaim_closed_nodes
    /// [`into_lazy_reclaim`]: crate::List::into_lazy_reclaim
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: DoublyList<_> = DoublyList::new();
    ///
    /// // growing will never invalidate indices
    /// let a = list.push_back('a');
    /// let b = list.push_back('b');
    /// let c = list.push_front('c');
    /// let d = list.push_front('d');
    ///
    /// assert_eq!(list.node_utilization().num_active_nodes, 4);
    /// assert_eq!(list.node_utilization().num_closed_nodes, 0);
    ///
    /// // make lazy
    /// let mut list: DoublyListLazy<_> = list.into_lazy_reclaim();
    ///
    /// // now removals will never lead to an automatic reorganization
    /// // hence a, b, c, d will never be invalidated unless they are removed
    /// let pop = list.pop_back();
    /// assert_eq!(pop, Some('b'));
    ///
    /// assert_eq!(list.node_utilization().num_active_nodes, 3);
    /// assert_eq!(list.node_utilization().num_closed_nodes, 1);
    ///
    /// // we can still use the indices to have constant time access to nodes
    ///
    /// assert_eq!(list.idx_err(a), None);
    /// assert_eq!(list.idx_err(b), Some(NodeIdxError::RemovedNode));
    /// assert_eq!(list.idx_err(c), None);
    /// assert_eq!(list.idx_err(d), None);
    ///
    /// assert_eq!(list.get(a), Some(&'a'));
    /// assert_eq!(list.get(b), None);
    /// assert_eq!(list.get(c), Some(&'c'));
    /// assert_eq!(list.get(d), Some(&'d'));
    ///
    /// // make auto again
    /// let mut list: DoublyList<_> = list.into_auto_reclaim();
    ///
    /// // now removals might lead to reorganization if node utilization
    /// // falls below a certain threshold (75% when D=2).
    /// let pop = list.remove(d);
    /// assert_eq!(pop, 'd');
    ///
    /// // 2 removed nodes reclaimed (b, d); 2 active nodes remains (c, a)
    /// assert_eq!(list.node_utilization().num_active_nodes, 2);
    /// assert_eq!(list.node_utilization().num_closed_nodes, 0);
    ///
    /// // node positions might have change during reorganization
    /// assert_eq!(list.idx_err(a), Some(NodeIdxError::ReorganizedCollection));
    /// assert_eq!(list.idx_err(b), Some(NodeIdxError::ReorganizedCollection));
    /// // or prior position does not belong to the storage any more
    /// assert_eq!(list.idx_err(c), Some(NodeIdxError::OutOfBounds));
    /// assert_eq!(list.idx_err(d), Some(NodeIdxError::OutOfBounds));
    ///
    /// // indices can no longer be used to access the elements
    /// assert_eq!(list.get(a), None);
    /// assert_eq!(list.get(b), None);
    /// assert_eq!(list.get(c), None);
    /// assert_eq!(list.get(d), None);
    ///
    /// // we can recollect valid indices if necessary
    /// let idx: Vec<_> = list.indices().collect();
    /// assert_eq!(list.get(idx[0]), Some(&'c'));
    /// assert_eq!(list.get(idx[1]), Some(&'a'));
    /// ```
    pub fn into_auto_reclaim(self) -> DoublyList<T, P> {
        self.into()
    }

    /// Transforms the list into auto memory reclaim mode, such as:
    /// * `DoublyListLazy` is transformed into `DoublyList`
    /// * `SinglyListLazy` is transformed into `SinglyList`
    ///
    /// This transformation has no cost, and can as well be reverted
    /// with no cost calling the [`into_lazy_reclaim`] method.
    ///
    /// The auto mode will reclaim memory whenever the ratio of
    /// active nodes to total used nodes; i.e., node utilization,
    /// falls below a certain threshold.
    ///
    /// [`reclaim_closed_nodes`]: crate::List::reclaim_closed_nodes
    /// [`into_lazy_reclaim`]: crate::List::into_lazy_reclaim
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: DoublyList<_> = DoublyList::new();
    ///
    /// // growing will never invalidate indices
    /// let a = list.push_back('a');
    /// let b = list.push_back('b');
    /// let c = list.push_front('c');
    /// let d = list.push_front('d');
    ///
    /// assert_eq!(list.node_utilization().num_active_nodes, 4);
    /// assert_eq!(list.node_utilization().num_closed_nodes, 0);
    ///
    /// // make lazy
    /// let mut list: DoublyListLazy<_> = list.into_lazy_reclaim();
    ///
    /// // now removals will never lead to an automatic reorganization
    /// // hence a, b, c, d will never be invalidated unless they are removed
    /// let pop = list.pop_back();
    /// assert_eq!(pop, Some('b'));
    ///
    /// assert_eq!(list.node_utilization().num_active_nodes, 3);
    /// assert_eq!(list.node_utilization().num_closed_nodes, 1);
    ///
    /// // we can still use the indices to have constant time access to nodes
    ///
    /// assert_eq!(list.idx_err(a), None);
    /// assert_eq!(list.idx_err(b), Some(NodeIdxError::RemovedNode));
    /// assert_eq!(list.idx_err(c), None);
    /// assert_eq!(list.idx_err(d), None);
    ///
    /// assert_eq!(list.get(a), Some(&'a'));
    /// assert_eq!(list.get(b), None);
    /// assert_eq!(list.get(c), Some(&'c'));
    /// assert_eq!(list.get(d), Some(&'d'));
    ///
    /// // make auto again
    /// let mut list: DoublyList<_> = list.into_auto_reclaim();
    ///
    /// // now removals might lead to reorganization if node utilization
    /// // falls below a certain threshold (75% when D=2).
    /// let pop = list.remove(d);
    /// assert_eq!(pop, 'd');
    ///
    /// // 2 removed nodes reclaimed (b, d); 2 active nodes remains (c, a)
    /// assert_eq!(list.node_utilization().num_active_nodes, 2);
    /// assert_eq!(list.node_utilization().num_closed_nodes, 0);
    ///
    /// // node positions might have change during reorganization
    /// assert_eq!(list.idx_err(a), Some(NodeIdxError::ReorganizedCollection));
    /// assert_eq!(list.idx_err(b), Some(NodeIdxError::ReorganizedCollection));
    /// // or prior position does not belong to the storage any more
    /// assert_eq!(list.idx_err(c), Some(NodeIdxError::OutOfBounds));
    /// assert_eq!(list.idx_err(d), Some(NodeIdxError::OutOfBounds));
    ///
    /// // indices can no longer be used to access the elements
    /// assert_eq!(list.get(a), None);
    /// assert_eq!(list.get(b), None);
    /// assert_eq!(list.get(c), None);
    /// assert_eq!(list.get(d), None);
    ///
    /// // we can recollect valid indices if necessary
    /// let idx: Vec<_> = list.indices().collect();
    /// assert_eq!(list.get(idx[0]), Some(&'c'));
    /// assert_eq!(list.get(idx[1]), Some(&'a'));
    /// ```
    pub fn into_auto_reclaim_with_threshold<const D: usize>(self) -> DoublyListThreshold<D, T, P> {
        self.into()
    }
}

impl<T, P> SinglyListLazy<T, P>
where
    P: PinnedVec<Node<Singly<T>>>,
{
    /// Transforms the list into auto memory reclaim mode, such as:
    /// * `DoublyListLazy` is transformed into `DoublyList`
    /// * `SinglyListLazy` is transformed into `SinglyList`
    ///
    /// This transformation has no cost, and can as well be reverted
    /// with no cost calling the [`into_lazy_reclaim`] method.
    ///
    /// The auto mode will reclaim memory whenever the ratio of
    /// active nodes to total used nodes; i.e., node utilization,
    /// falls below a certain threshold.
    ///
    /// [`reclaim_closed_nodes`]: crate::List::reclaim_closed_nodes
    /// [`into_lazy_reclaim`]: crate::List::into_lazy_reclaim
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: DoublyList<_> = DoublyList::new();
    ///
    /// // growing will never invalidate indices
    /// let a = list.push_back('a');
    /// let b = list.push_back('b');
    /// let c = list.push_front('c');
    /// let d = list.push_front('d');
    ///
    /// assert_eq!(list.node_utilization().num_active_nodes, 4);
    /// assert_eq!(list.node_utilization().num_closed_nodes, 0);
    ///
    /// // make lazy
    /// let mut list: DoublyListLazy<_> = list.into_lazy_reclaim();
    ///
    /// // now removals will never lead to an automatic reorganization
    /// // hence a, b, c, d will never be invalidated unless they are removed
    /// let pop = list.pop_back();
    /// assert_eq!(pop, Some('b'));
    ///
    /// assert_eq!(list.node_utilization().num_active_nodes, 3);
    /// assert_eq!(list.node_utilization().num_closed_nodes, 1);
    ///
    /// // we can still use the indices to have constant time access to nodes
    ///
    /// assert_eq!(list.idx_err(a), None);
    /// assert_eq!(list.idx_err(b), Some(NodeIdxError::RemovedNode));
    /// assert_eq!(list.idx_err(c), None);
    /// assert_eq!(list.idx_err(d), None);
    ///
    /// assert_eq!(list.get(a), Some(&'a'));
    /// assert_eq!(list.get(b), None);
    /// assert_eq!(list.get(c), Some(&'c'));
    /// assert_eq!(list.get(d), Some(&'d'));
    ///
    /// // make auto again
    /// let mut list: DoublyList<_> = list.into_auto_reclaim();
    ///
    /// // now removals might lead to reorganization if node utilization
    /// // falls below a certain threshold (75% when D=2).
    /// let pop = list.remove(d);
    /// assert_eq!(pop, 'd');
    ///
    /// // 2 removed nodes reclaimed (b, d); 2 active nodes remains (c, a)
    /// assert_eq!(list.node_utilization().num_active_nodes, 2);
    /// assert_eq!(list.node_utilization().num_closed_nodes, 0);
    ///
    /// // node positions might have change during reorganization
    /// assert_eq!(list.idx_err(a), Some(NodeIdxError::ReorganizedCollection));
    /// assert_eq!(list.idx_err(b), Some(NodeIdxError::ReorganizedCollection));
    /// // or prior position does not belong to the storage any more
    /// assert_eq!(list.idx_err(c), Some(NodeIdxError::OutOfBounds));
    /// assert_eq!(list.idx_err(d), Some(NodeIdxError::OutOfBounds));
    ///
    /// // indices can no longer be used to access the elements
    /// assert_eq!(list.get(a), None);
    /// assert_eq!(list.get(b), None);
    /// assert_eq!(list.get(c), None);
    /// assert_eq!(list.get(d), None);
    ///
    /// // we can recollect valid indices if necessary
    /// let idx: Vec<_> = list.indices().collect();
    /// assert_eq!(list.get(idx[0]), Some(&'c'));
    /// assert_eq!(list.get(idx[1]), Some(&'a'));
    /// ```
    pub fn into_auto_reclaim(self) -> SinglyList<T, P> {
        self.into()
    }

    /// Transforms the list into auto memory reclaim mode, such as:
    /// * `DoublyListLazy` is transformed into `DoublyList`
    /// * `SinglyListLazy` is transformed into `SinglyList`
    ///
    /// This transformation has no cost, and can as well be reverted
    /// with no cost calling the [`into_lazy_reclaim`] method.
    ///
    /// The auto mode will reclaim memory whenever the ratio of
    /// active nodes to total used nodes; i.e., node utilization,
    /// falls below a certain threshold.
    ///
    /// [`reclaim_closed_nodes`]: crate::List::reclaim_closed_nodes
    /// [`into_lazy_reclaim`]: crate::List::into_lazy_reclaim
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: DoublyList<_> = DoublyList::new();
    ///
    /// // growing will never invalidate indices
    /// let a = list.push_back('a');
    /// let b = list.push_back('b');
    /// let c = list.push_front('c');
    /// let d = list.push_front('d');
    ///
    /// assert_eq!(list.node_utilization().num_active_nodes, 4);
    /// assert_eq!(list.node_utilization().num_closed_nodes, 0);
    ///
    /// // make lazy
    /// let mut list: DoublyListLazy<_> = list.into_lazy_reclaim();
    ///
    /// // now removals will never lead to an automatic reorganization
    /// // hence a, b, c, d will never be invalidated unless they are removed
    /// let pop = list.pop_back();
    /// assert_eq!(pop, Some('b'));
    ///
    /// assert_eq!(list.node_utilization().num_active_nodes, 3);
    /// assert_eq!(list.node_utilization().num_closed_nodes, 1);
    ///
    /// // we can still use the indices to have constant time access to nodes
    ///
    /// assert_eq!(list.idx_err(a), None);
    /// assert_eq!(list.idx_err(b), Some(NodeIdxError::RemovedNode));
    /// assert_eq!(list.idx_err(c), None);
    /// assert_eq!(list.idx_err(d), None);
    ///
    /// assert_eq!(list.get(a), Some(&'a'));
    /// assert_eq!(list.get(b), None);
    /// assert_eq!(list.get(c), Some(&'c'));
    /// assert_eq!(list.get(d), Some(&'d'));
    ///
    /// // make auto again
    /// let mut list: DoublyList<_> = list.into_auto_reclaim();
    ///
    /// // now removals might lead to reorganization if node utilization
    /// // falls below a certain threshold (75% when D=2).
    /// let pop = list.remove(d);
    /// assert_eq!(pop, 'd');
    ///
    /// // 2 removed nodes reclaimed (b, d); 2 active nodes remains (c, a)
    /// assert_eq!(list.node_utilization().num_active_nodes, 2);
    /// assert_eq!(list.node_utilization().num_closed_nodes, 0);
    ///
    /// // node positions might have change during reorganization
    /// assert_eq!(list.idx_err(a), Some(NodeIdxError::ReorganizedCollection));
    /// assert_eq!(list.idx_err(b), Some(NodeIdxError::ReorganizedCollection));
    /// // or prior position does not belong to the storage any more
    /// assert_eq!(list.idx_err(c), Some(NodeIdxError::OutOfBounds));
    /// assert_eq!(list.idx_err(d), Some(NodeIdxError::OutOfBounds));
    ///
    /// // indices can no longer be used to access the elements
    /// assert_eq!(list.get(a), None);
    /// assert_eq!(list.get(b), None);
    /// assert_eq!(list.get(c), None);
    /// assert_eq!(list.get(d), None);
    ///
    /// // we can recollect valid indices if necessary
    /// let idx: Vec<_> = list.indices().collect();
    /// assert_eq!(list.get(idx[0]), Some(&'c'));
    /// assert_eq!(list.get(idx[1]), Some(&'a'));
    /// ```
    pub fn into_auto_reclaim_with_threshold<const D: usize>(self) -> SinglyListThreshold<D, T, P> {
        self.into()
    }
}
