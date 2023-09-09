use crate::{linked_list::LinkedList, node::LinkedListNode};
use orx_imp_vec::prelude::*;

impl<'a, T> LinkedList<'a, T> {
    /// Creates an empty LinkedList with default pinned vector.
    ///
    /// Default underlying pinned vector is the `SplitVec` with default growth strategy.
    ///
    /// *See [SplitVec(https://crates.io/crates/orx-split-vec) for details.*
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back('a');
    /// ```
    pub fn new() -> Self {
        let imp: ImpVec<_> = SplitVec::new().into();
        imp.push(LinkedListNode::back_front_node());
        Self {
            vec: imp,
            len: 0,
            memory_utilization: Default::default(),
        }
    }
    /// Creates an empty LinkedList with default pinned vector
    /// having the given `initial_capacity`.
    ///
    /// Default underlying pinned vector is the `SplitVec` with default growth strategy.
    ///
    /// *See [SplitVec(https://crates.io/crates/orx-split-vec) for details.*
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_initial_capacity(8);
    /// list.push_back('a');
    /// ```
    pub fn with_initial_capacity(initial_capacity: usize) -> Self {
        let imp: ImpVec<_> = SplitVec::with_initial_capacity(initial_capacity).into();
        imp.push(LinkedListNode::back_front_node());
        Self {
            vec: imp,
            len: 0,
            memory_utilization: Default::default(),
        }
    }
}

impl<'a, T> LinkedList<'a, T, FixedVec<LinkedListNode<'a, T>>> {
    /// Creates an empty LinkedList with fixed capacity.
    ///
    /// `FixedVec` is the most efficient `PinnedVec` implementation which can be
    /// used as the underlying data structure; however, it panics if the memory
    /// usage exceeds the given `fixed_capacity`.
    ///
    /// It implements the `room` method to reveal the available space in number
    /// of elements.
    ///
    /// *The `ImpVec` of the linked list allowing to build internal links
    /// uses a `FixedVec: PinnedVec`, see [`FixedVec`](https://crates.io/crates/orx-fixed-vec) for details.*
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let list: LinkedList<char, FixedVec<LinkedListNode<char>>>
    ///     = LinkedList::with_fixed_capacity(128);
    ///
    /// // equivalent brief type alias
    /// let list: LinkedListFixed<char>
    ///     = LinkedList::with_fixed_capacity(128);
    ///
    /// ```
    pub fn with_fixed_capacity(fixed_capacity: usize) -> Self {
        let imp: ImpVec<_, _> = FixedVec::new(fixed_capacity).into();
        imp.push(LinkedListNode::back_front_node());
        Self {
            vec: imp,
            len: 0,
            memory_utilization: Default::default(),
        }
    }
}
impl<'a, T> LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, Doubling>> {
    /// Creates an empty LinkedList where new allocations are doubled every time
    /// the vector reaches its capacity.
    ///
    /// * `first_fragment_capacity` determines the capacity of the first fragment
    /// of the underlying split vector.
    ///
    /// *The `ImpVec` of the linked list allowing to build internal links
    /// uses a `SplitVec: PinnedVec` with `Doubling` growth strategy.
    /// See [`SplitVec`](https://crates.io/crates/orx-split-vec) for details.*
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let list: LinkedList<char, SplitVec<LinkedListNode<char>, Doubling>>
    ///     = LinkedList::with_doubling_growth(4);
    ///
    /// // equivalent brief type alias
    /// let list: LinkedListDoubling<char>
    ///     = LinkedList::with_doubling_growth(128);
    /// ```
    pub fn with_doubling_growth(first_fragment_capacity: usize) -> Self {
        let imp: ImpVec<_, _> = SplitVec::with_doubling_growth(first_fragment_capacity).into();
        imp.push(LinkedListNode::back_front_node());
        Self {
            vec: imp,
            memory_utilization: Default::default(),
            len: 0,
        }
    }
}
impl<'a, T> LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, Linear>> {
    /// Creates an empty LinkedList where new allocations are always the same
    /// and equal to the initial capacity of the vector.
    ///
    /// * `constant_fragment_capacity` determines the capacity of the first fragment
    /// and every succeeding fragment of the underlying split vector.
    ///
    /// *The `ImpVec` of the linked list allowing to build internal links
    /// uses a `SplitVec: PinnedVec` with `Linear` growth strategy.
    /// See [`SplitVec`](https://crates.io/crates/orx-split-vec) for details.*
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let list: LinkedList<char, SplitVec<LinkedListNode<char>, Linear>>
    ///     = LinkedList::with_linear_growth(32);
    ///
    /// // equivalent brief type alias
    /// let list: LinkedListLinear<char>
    ///     = LinkedList::with_linear_growth(128);
    /// ```
    pub fn with_linear_growth(constant_fragment_capacity: usize) -> Self {
        let imp: ImpVec<_, _> = SplitVec::with_linear_growth(constant_fragment_capacity).into();
        imp.push(LinkedListNode::back_front_node());
        Self {
            vec: imp,
            memory_utilization: Default::default(),
            len: 0,
        }
    }
}
impl<'a, T> LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, Exponential>> {
    /// Creates an empty LinkedList where new allocations are exponentially increased
    /// every time the vector reaches its capacity.
    ///
    /// * `first_fragment_capacity` determines the capacity of the first fragment
    /// of the underlying split vector.
    /// * `growth_coefficient` determines the exponential growth rate of the succeeding
    /// fragments of the split vector.
    ///
    /// *The `ImpVec` of the linked list allowing to build internal links
    /// uses a `SplitVec: PinnedVec` with `Exponential` growth strategy.
    /// See [`SplitVec`](https://crates.io/crates/orx-split-vec) for details.*
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let list: LinkedList<char, SplitVec<LinkedListNode<char>, Exponential>>
    ///     = LinkedList::with_exponential_growth(4, 1.5);
    ///
    /// // equivalent brief type alias
    /// let list: LinkedListExponential<char>
    ///     = LinkedList::with_exponential_growth(4, 1.5);
    /// ```
    pub fn with_exponential_growth(
        first_fragment_capacity: usize,
        growth_coefficient: f32,
    ) -> Self {
        let imp: ImpVec<_, _> =
            SplitVec::with_exponential_growth(first_fragment_capacity, growth_coefficient).into();
        imp.push(LinkedListNode::back_front_node());
        Self {
            vec: imp,
            memory_utilization: Default::default(),
            len: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let mut list = LinkedList::new();
        list.push_back('a');

        let list: LinkedList<char> = list;
        let list: LinkedList<char, SplitVec<LinkedListNode<char>>> = list;
        let list: LinkedList<char, SplitVec<LinkedListNode<char>, Doubling>> = list;
        assert_eq!(1, list.vec.fragments().len());
        assert_eq!(4, list.vec.fragments()[0].capacity());
    }

    #[test]
    fn with_initial_capacity() {
        let mut list = LinkedList::with_initial_capacity(10);
        list.push_back('a');

        let list: LinkedList<char> = list;
        let list: LinkedList<char, SplitVec<LinkedListNode<char>>> = list;
        let list: LinkedList<char, SplitVec<LinkedListNode<char>, Doubling>> = list;
        assert_eq!(1, list.vec.fragments().len());
        assert_eq!(10, list.vec.fragments()[0].capacity());
    }
}
