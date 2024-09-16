use super::List;
use crate::variant::ListVariant;
use orx_selfref_col::MemoryPolicy;

impl<V, M> List<V, M>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
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
}
