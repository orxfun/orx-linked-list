use crate::{linked_list::LinkedList, node::LinkedListNode};
use orx_imp_vec::prelude::*;

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
        let imp: ImpVec<_, _> = FixedVec::new(fixed_capacity + 1).into();
        imp.push(LinkedListNode::back_front_node());
        Self {
            imp,
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
            imp,
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
            imp,
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
            imp,
            memory_utilization: Default::default(),
            len: 0,
        }
    }
}
