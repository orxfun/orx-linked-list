use crate::{
    list::List, variant::ListVariant, DoublyList, DoublyListLazy, DoublyListThreshold, SinglyList,
    SinglyListLazy, SinglyListThreshold,
};
use orx_fixed_vec::FixedVec;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node, Refs, SelfRefCol};
use orx_split_vec::{Doubling, Linear, Recursive, SplitVec};

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

// pinned-vec variants

impl<V, M, P> List<V, M, P>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
    P: PinnedVec<Node<V>>,
{
    fn from_empty_pinned_vec(nodes: P) -> Self {
        assert!(nodes.is_empty());
        let ends = V::Ends::empty();
        let col = SelfRefCol::from((nodes, ends));
        Self(col)
    }
}

impl<V, M> List<V, M, FixedVec<Node<V>>>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
{
    /// Creates a linked list that uses a [`FixedVec<T>`]((https://docs.rs/orx-fixed-vec/latest/orx_fixed_vec/)) as the underlying storage.
    pub fn with_fixed_capacity(fixed_capacity: usize) -> Self {
        Self::from_empty_pinned_vec(FixedVec::new(fixed_capacity))
    }
}

impl<V, M> List<V, M, SplitVec<Node<V>, Doubling>>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
{
    /// Creates a linked list that uses a [`SplitVec<T, Doubling>`](https://docs.rs/orx-split-vec/latest/orx_split_vec/struct.Doubling.html) as the underlying storage.
    pub fn with_doubling_growth() -> Self {
        Self::from_empty_pinned_vec(SplitVec::with_doubling_growth())
    }
}

impl<V, M> List<V, M, SplitVec<Node<V>, Recursive>>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
{
    /// Creates a linked list that uses a [`SplitVec<T, Recursive>`](https://docs.rs/orx-split-vec/latest/orx_split_vec/struct.Recursive.html) as the underlying storage.
    pub fn with_recursive_growth() -> Self {
        Self::from_empty_pinned_vec(SplitVec::with_recursive_growth())
    }
}

impl<V, M> List<V, M, SplitVec<Node<V>, Linear>>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
{
    /// Creates a linked list that uses a [`SplitVec<T, Linear>`](https://docs.rs/orx-split-vec/latest/orx_split_vec/struct.Linear.html) as the underlying storage.
    ///
    /// Each fragment will have a capacity of 2 ^ constant_fragment_capacity_exponent.
    pub fn with_linear_growth(constant_fragment_capacity_exponent: usize) -> Self {
        Self::from_empty_pinned_vec(SplitVec::with_linear_growth(
            constant_fragment_capacity_exponent,
        ))
    }
}
