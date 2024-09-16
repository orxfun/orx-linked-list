use crate::{
    list::List, DoublyList, DoublyListLazy, DoublyListThreshold, SinglyList, SinglyListLazy,
    SinglyListThreshold,
};
use orx_selfref_col::SelfRefCol;

// singly

impl<T> SinglyList<T> {
    /// Creates an empty singly linked list with default memory reclaim policy.
    pub fn new() -> Self {
        Self(SelfRefCol::new())
    }

    /// Creates an empty singly linked list with custom memory reclaim on threshold policy:
    /// * memory of removed nodes are automatically reclaimed when the ratio of closed nodes to all nodes exceeds one over 2^D:
    ///   * when D = 0: memory will be reclaimed when utilization is below 0.00% (equivalent to Lazy).
    ///   * when D = 1: memory will be reclaimed when utilization is below 50.00%.
    ///   * when D = 2: memory will be reclaimed when utilization is below 75.00%.
    ///   * when D = 3: memory will be reclaimed when utilization is below 87.50%.
    ///   * when D = 4: memory will be reclaimed when utilization is below 93.75%.
    pub fn with_threshold_reclaimer<const D: usize>() -> SinglyListThreshold<D, T> {
        List(SelfRefCol::new())
    }
}
impl<T> Default for SinglyList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SinglyListLazy<T> {
    /// Creates an empty singly linked list with lazy memory reclaim policy.
    ///
    /// Memory of removed nodes are never reclaimed implicitly, the caller can explicitly reclaim by calling `reclaim_closed_nodes`.
    ///
    /// This also guarantees that indices will never be invalidated implicitly.
    pub fn new() -> Self {
        Self(SelfRefCol::new())
    }
}
impl<T> Default for SinglyListLazy<T> {
    fn default() -> Self {
        Self::new()
    }
}

// doubly

impl<T> DoublyList<T> {
    /// Creates an empty doubly linked list with default memory reclaim policy.
    pub fn new() -> Self {
        Self(SelfRefCol::new())
    }

    /// Creates an empty doubly linked list with custom memory reclaim on threshold policy:
    /// * memory of removed nodes are automatically reclaimed when the ratio of closed nodes to all nodes exceeds one over 2^D:
    ///   * when D = 0: memory will be reclaimed when utilization is below 0.00% (equivalent to Lazy).
    ///   * when D = 1: memory will be reclaimed when utilization is below 50.00%.
    ///   * when D = 2: memory will be reclaimed when utilization is below 75.00%.
    ///   * when D = 3: memory will be reclaimed when utilization is below 87.50%.
    ///   * when D = 4: memory will be reclaimed when utilization is below 93.75%.
    pub fn with_threshold_reclaimer<const D: usize>() -> DoublyListThreshold<D, T> {
        List(SelfRefCol::new())
    }
}
impl<T> Default for DoublyList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DoublyListLazy<T> {
    /// Creates an empty doubly linked list with lazy memory reclaim policy.
    ///
    /// Memory of removed nodes are never reclaimed implicitly, the caller can explicitly reclaim by calling `reclaim_closed_nodes`.
    ///
    /// This also guarantees that indices will never be invalidated implicitly.
    pub fn new() -> Self {
        Self(SelfRefCol::new())
    }
}
impl<T> Default for DoublyListLazy<T> {
    fn default() -> Self {
        Self::new()
    }
}
