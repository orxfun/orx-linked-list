use crate::{linked_list::LinkedList, mem::MemoryUtilization, node::LinkedListNode};
use orx_imp_vec::{
    prelude::{CustomGrowth, DoublingGrowth, ExponentialGrowth, Fragment, LinearGrowth, SplitVec},
    ImpVec,
};
use std::rc::Rc;

impl<'a, T> LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, DoublingGrowth>> {
    /// Creates an empty LinkedList where new allocations are doubled every time
    /// the vector reaches its capacity.
    ///
    /// * `first_fragment_capacity` determines the capacity of the first fragment
    /// of the underlying split vector.
    /// * `memory_utilization` defines the memory utilization strategy of the linked
    /// list, see `MemoryUtilization` for details.
    ///
    /// *The `ImpVec` of the linked list allowing to build internal links
    /// uses a `SplitVec: PinnedVec` with `DoublingGrowth` growth strategy.
    /// See [`SplitVec`](https://crates.io/crates/orx-split-vec) for details.*
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_doubling_growth(4, MemoryUtilization::Lazy);
    ///
    /// for i in 0..8 {
    ///     list.push_back(i);
    /// }
    /// assert_eq!(8, list.len());
    /// ```
    pub fn with_doubling_growth(
        first_fragment_capacity: usize,
        memory_utilization: MemoryUtilization,
    ) -> Self {
        let memory_utilization = memory_utilization.into_valid();
        let imp: ImpVec<_, _> = SplitVec::with_doubling_growth(first_fragment_capacity).into();
        imp.push(LinkedListNode::back_front_node());
        Self {
            imp,
            memory_utilization,
            len: 0,
        }
    }
}
impl<'a, T> LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, LinearGrowth>> {
    /// Creates an empty LinkedList where new allocations are always the same
    /// and equal to the initial capacity of the vector.
    ///
    /// * `constant_fragment_capacity` determines the capacity of the first fragment
    /// and every succeeding fragment of the underlying split vector.
    /// * `memory_utilization` defines the memory utilization strategy of the linked
    /// list, see `MemoryUtilization` for details.
    ///
    /// *The `ImpVec` of the linked list allowing to build internal links
    /// uses a `SplitVec: PinnedVec` with `LinearGrowth` growth strategy.
    /// See [`SplitVec`](https://crates.io/crates/orx-split-vec) for details.*
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(5, MemoryUtilization::Eager);
    ///
    /// for i in 0..8 {
    ///     list.push_back(i);
    /// }
    /// assert_eq!(8, list.len());
    /// ```
    pub fn with_linear_growth(
        constant_fragment_capacity: usize,
        memory_utilization: MemoryUtilization,
    ) -> Self {
        let memory_utilization = memory_utilization.into_valid();
        let imp: ImpVec<_, _> = SplitVec::with_linear_growth(constant_fragment_capacity).into();
        imp.push(LinkedListNode::back_front_node());
        Self {
            imp,
            memory_utilization,
            len: 0,
        }
    }
}
impl<'a, T> LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, ExponentialGrowth>> {
    /// Creates an empty LinkedList where new allocations are exponentially increased
    /// every time the vector reaches its capacity.
    ///
    /// * `first_fragment_capacity` determines the capacity of the first fragment
    /// of the underlying split vector.
    /// * `growth_coefficient` determines the exponential growth rate of the succeeding
    /// fragments of the split vector.
    /// * `memory_utilization` defines the memory utilization strategy of the linked
    /// list, see `MemoryUtilization` for details.
    ///
    /// *The `ImpVec` of the linked list allowing to build internal links
    /// uses a `SplitVec: PinnedVec` with `ExponentialGrowth` growth strategy.
    /// See [`SplitVec`](https://crates.io/crates/orx-split-vec) for details.*
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_exponential_growth(4, 1.5, MemoryUtilization::default());
    ///
    /// for i in 0..8 {
    ///     list.push_back(i);
    /// }
    /// assert_eq!(8, list.len());
    /// ```
    pub fn with_exponential_growth(
        first_fragment_capacity: usize,
        growth_coefficient: f32,
        memory_utilization: MemoryUtilization,
    ) -> Self {
        let memory_utilization = memory_utilization.into_valid();
        let imp: ImpVec<_, _> =
            SplitVec::with_exponential_growth(first_fragment_capacity, growth_coefficient).into();
        imp.push(LinkedListNode::back_front_node());
        Self {
            imp,
            memory_utilization,
            len: 0,
        }
    }
}

pub(crate) type GetCapacityOfNewFragment<T> = dyn Fn(&[Fragment<T>]) -> usize;
impl<'a, T>
    LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, CustomGrowth<LinkedListNode<'a, T>>>>
{
    /// Creates an empty LinkedList where new allocations are determined explicitly
    /// by the passed in function.
    ///
    /// * `get_capacity_of_new_fragment` determines the capacity of succeeding
    /// fragments as a function of already created and filled fragments.
    /// * `memory_utilization` defines the memory utilization strategy of the linked
    /// list, see `MemoryUtilization` for details.
    ///
    /// *The `ImpVec` of the linked list allowing to build internal links
    /// uses a `SplitVec: PinnedVec` with `CustomGrowth` growth strategy.
    /// See [`SplitVec`](https://crates.io/crates/orx-split-vec) for details.*
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    /// use std::rc::Rc;
    ///
    /// let mut list =
    ///     LinkedList::with_custom_growth_function(Rc::new(|fragments: &[Fragment<_>]| {
    ///         if fragments.len() % 2 == 0 {
    ///             2
    ///         } else {
    ///             8
    ///         }
    ///     }), MemoryUtilization::WithThreshold(0.25));
    ///
    /// for i in 0..8 {
    ///     list.push_back(i);
    /// }
    /// assert_eq!(8, list.len());
    /// ```
    pub fn with_custom_growth_function(
        get_capacity_of_new_fragment: Rc<GetCapacityOfNewFragment<LinkedListNode<'a, T>>>,
        memory_utilization: MemoryUtilization,
    ) -> Self {
        let memory_utilization = memory_utilization.into_valid();
        let imp: ImpVec<_, _> =
            SplitVec::with_custom_growth_function(get_capacity_of_new_fragment).into();
        imp.push(LinkedListNode::back_front_node());
        Self {
            imp,
            memory_utilization,
            len: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubling() {
        let list = LinkedList::<i32, _>::with_doubling_growth(10, MemoryUtilization::Eager);
        assert_eq!(list.imp.fragments().len(), 1);
        assert_eq!(list.imp.fragments()[0].capacity(), 10);
        assert_eq!(list.memory_utilization, MemoryUtilization::Eager);
    }

    #[test]
    fn linear() {
        let list = LinkedList::<i32, _>::with_linear_growth(10, MemoryUtilization::Lazy);
        assert_eq!(list.imp.fragments().len(), 1);
        assert_eq!(list.imp.fragments()[0].capacity(), 10);
        assert_eq!(list.memory_utilization, MemoryUtilization::Lazy);
    }

    #[test]
    fn exponential() {
        let list = LinkedList::<i32, _>::with_exponential_growth(
            10,
            1.5,
            MemoryUtilization::WithThreshold(0.5),
        );
        assert_eq!(list.imp.fragments().len(), 1);
        assert_eq!(list.imp.fragments()[0].capacity(), 10);
        assert_eq!(
            list.memory_utilization,
            MemoryUtilization::WithThreshold(0.5)
        );
    }

    #[test]
    fn custom() {
        let list = LinkedList::<i32, _>::with_custom_growth_function(
            Rc::new(
                |fragments: &[Fragment<_>]| {
                    if fragments.len() % 2 == 0 {
                        3
                    } else {
                        8
                    }
                },
            ),
            MemoryUtilization::Lazy,
        );
        assert_eq!(list.imp.fragments().len(), 1);
        assert_eq!(list.imp.fragments()[0].capacity(), 3);
        assert_eq!(list.memory_utilization, MemoryUtilization::Lazy);
    }
}
