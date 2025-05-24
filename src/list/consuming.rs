use super::List;
use crate::variant::ListVariant;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node};

impl<V, M, P> List<V, M, P>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
    P: PinnedVec<Node<V>>,
{
    /// Returns an arbitrary order consuming iterator of owned elements of the list.
    ///
    /// Note that the iterator created by `into_iter_x` is often faster than that created by `into_iter`;
    /// and hence, can be preferred whenever the iteration order does not matter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// // a -> b -> c
    /// list.push_front('c');
    /// list.push_front('b');
    /// list.push_back('x');
    /// list.push_front('a');
    /// list.pop_back();
    ///
    /// let mut vec: Vec<_> = list.into_iter_x().collect();
    ///
    /// // although deterministic depending on order of mutations,
    /// // the order can be considered deterministic.
    /// assert_eq!(vec.as_slice(), &['c', 'b', 'a']);
    ///
    /// vec.sort();
    /// assert_eq!(vec.as_slice(), &['a', 'b', 'c']);
    /// ```
    pub fn into_iter_x(self) -> impl Iterator<Item = V::Item> {
        let (nodes, _, _) = self.0.into_inner().0.into_inner();
        nodes.into_iter().filter_map(|x| x.into_data())
    }

    /// Consumes the linked list and creates a parallel iterator over owned elements in **arbitrary order**.
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
    /// let new_list = || DoublyList::from_iter(0..1024);
    ///
    /// let expected: usize = new_list().iter_x().sum();
    ///
    /// let sum = new_list().into_par_x().sum(); // parallelized computation
    /// assert_eq!(expected, sum);
    ///
    /// let sum = new_list().into_par_x().num_threads(4).sum(); // using at most 4 threads
    /// assert_eq!(expected, sum);
    ///
    /// let sum_doubles = new_list().into_par_x().map(|x| x * 2).sum();
    /// assert_eq!(2 * expected, sum_doubles);
    ///
    /// let expected: usize = new_list().into_iter_x().filter(|x| x % 2 == 0).sum();
    /// let sum_evens = new_list().into_par_x().filter(|x| x % 2 == 0).sum();
    /// std::dbg!(sum_evens, expected);
    /// ```
    #[cfg(feature = "orx-parallel")]
    pub fn into_par_x(self) -> impl orx_parallel::ParIter<Item = V::Item>
    where
        V::Item: Send + Sync + Clone,
        Node<V>: Send + Sync,
        P: orx_concurrent_iter::IntoConcurrentIter<Item = Node<V>>,
    {
        use orx_parallel::*;
        let (pinned, _, _) = self.0.into_inner().0.into_inner();
        pinned.into_par().filter_map(|x| x.into_data())
    }
}
