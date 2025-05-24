use super::List;
use crate::variant::ListVariant;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, MemoryState, Node, Utilization};

impl<V, M, P> List<V, M, P>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
    P: PinnedVec<Node<V>>,
{
    /// ***O(1)*** Returns the number of elements in the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// assert_eq!(0, list.len());
    ///
    /// list.push_back('a');
    /// list.push_front('b');
    /// _ = list.pop_back();
    /// list.push_back('c');
    ///
    /// assert_eq!(2, list.len());
    /// ```
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the number of elements in the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = SinglyList::new();
    ///
    /// assert!(list.is_empty());
    ///
    /// list.push_front('a');
    /// assert!(!list.is_empty());
    ///
    /// _ = list.pop_front();
    /// assert!(list.is_empty());
    /// ```
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a key representing the memory state of the list.
    ///
    /// The list's memory state changes only when its nodes are reorganized.
    ///
    /// For [`SinglyListLazy`] or [`DoublyListLazy`]:
    /// * a node reorganization never happens implicitly,
    /// * it can only be triggered manually by calling `reclaim_closed_nodes`.
    ///
    /// For [`SinglyList`] and [`DoublyList`] with default memory policy:
    /// * a node reorganization might be triggered on methods that remove nodes from the list such as `pop_front` or `remove`,
    /// * a removal leads to a node reorganization if the ratio of closed nodes to all nodes exceeds `25%` (see [`Utilization`]);
    ///
    /// A node reorganization does not necessarily lead to a change in memory state; however, it is likely.
    ///
    /// Memory state is critical due to the following:
    /// * let `indices` be a collection of node indices ([`DoublyIdx`] or [`SinglyIdx`]) which are taken when the memory state was "s1";
    ///   * note that all methods that add an element to the list returns its index, such as `push_back` or `insert_at`,
    ///   * further note that these growth methods never change the state.
    /// * as long as the memory state does not change (stays as "s1"):
    ///   * we can use all of the `indices` to safely access elements in constant time,
    ///   * or use them in constant time mutation methods such as `insert_after` or `remove`.
    /// * at some point, the memory state may change to "s2" due to:
    ///   * manually calling `reclaim_closed_nodes` on a lazy memory policy list, or
    ///   * due to many removals from the list,
    /// * then, all of the `indices` obtained beforehand are invalidated,
    ///   * constant time methods requiring indices will safely fail with a proper error.
    ///
    /// [`SinglyList`]: crate::SinglyList
    /// [`DoublyList`]: crate::DoublyList
    /// [`SinglyListLazy`]: crate::SinglyListLazy
    /// [`DoublyListLazy`]: crate::DoublyListLazy
    /// [`DoublyIdx`]: crate::DoublyIdx
    /// [`SinglyIdx`]: crate::SinglyIdx
    #[inline(always)]
    pub fn memory_state(&self) -> MemoryState {
        self.0.memory_state()
    }

    /// Returns the node utilization of the underlying storage of the linked list.
    pub fn node_utilization(&self) -> Utilization {
        self.0.utilization()
    }

    /// Creates an arbitrary order iterator on elements of the list.
    ///
    /// Note that the iterator created by `iter_x` is often faster than that created by `iter`;
    /// and hence, can be preferred whenever the iteration order does not matter.
    pub fn iter_x(&self) -> impl Iterator<Item = &V::Item> {
        self.0.nodes().iter().filter_map(|x| x.data())
    }

    /// Creates a parallel iterator over references to the elements of the linked list in **arbitrary order**.
    ///
    /// Please see [`ParIter`] for details of the parallel computation.
    /// In brief, computation is defined as chain of iterator transformations and parallelization
    /// is handled by the underlying parallel executor.
    ///
    /// Required **orx-parallel** feature.
    ///
    /// [`ParIter`]: orx_parallel::ParIter
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let list: DoublyList<_> = (0..1024).collect();
    ///
    /// let expected: usize = list.iter_x().sum();
    ///
    /// let sum = list.par_x().sum(); // parallelized computation
    /// assert_eq!(expected, sum);
    ///
    /// let sum = list.par_x().num_threads(4).sum(); // using at most 4 threads
    /// assert_eq!(expected, sum);
    ///
    /// let sum_doubles = list.par_x().map(|x| x * 2).sum();
    /// assert_eq!(2 * expected, sum_doubles);
    ///
    /// let expected: usize = list.iter_x().filter(|x| *x % 2 == 0).sum();
    /// let sum_evens = list.par_x().filter(|x| *x % 2 == 0).sum();
    /// std::dbg!(sum_evens, expected);
    /// ```
    #[cfg(feature = "orx-parallel")]
    pub fn par_x(&self) -> impl orx_parallel::ParIter<Item = &V::Item>
    where
        V::Item: Send + Sync,
        Node<V>: Send + Sync,
        for<'a> &'a P: orx_concurrent_iter::IntoConcurrentIter<Item = &'a Node<V>>,
    {
        use orx_parallel::*;
        let pinned = self.0.nodes();
        pinned.par().filter_map(|x| x.data())
    }
}
