use crate::{node::LinkedListNode, LinkedList};
use orx_imp_vec::prelude::FixedVec;

impl<'a, T> LinkedList<'a, T, FixedVec<LinkedListNode<'a, T>>> {
    /// Returns the available room for new items.
    ///
    /// # Examples
    ///
    /// ```
    /// pub use orx_linked_list::prelude::*;
    ///
    /// let mut list =
    ///     LinkedList::with_fixed_capacity(4).with_memory_utilization(MemoryUtilization::Lazy);
    /// assert_eq!(4, list.room());
    ///
    /// list.push_back(1);
    /// list.push_back(1);
    /// list.push_back(1);
    /// list.push_back(1);
    /// assert_eq!(0, list.room());
    ///
    /// _ = list.pop_back();
    /// _ = list.pop_back();
    /// _ = list.pop_back();
    /// assert_eq!(0, list.room());
    ///
    /// list.memory_reclaim();
    /// assert_eq!(3, list.room());
    /// ```
    ///
    /// # Safety
    ///
    /// Note that since `FixedVec` has a strict capacity; pushing to the list
    /// while there is no room leads to a panic.
    pub fn room(&self) -> usize {
        self.imp.room()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MemoryUtilization;
    pub use orx_imp_vec::prelude::*;

    #[test]
    fn room() {
        let mut list =
            LinkedList::with_fixed_capacity(4).with_memory_utilization(MemoryUtilization::Lazy);
        assert_eq!(4, list.room());

        list.push_back(1);
        list.push_back(2);
        assert_eq!(2, list.room());

        _ = list.pop_back();
        assert_eq!(2, list.room());

        _ = list.pop_front();
        assert_eq!(2, list.room());

        list.memory_reclaim();
        assert_eq!(4, list.room());
    }

    #[test]
    #[should_panic]
    fn push_when_no_room() {
        let mut list =
            LinkedList::with_fixed_capacity(4).with_memory_utilization(MemoryUtilization::Lazy);
        assert_eq!(4, list.room());

        list.push_back(1);
        list.push_back(2);
        list.push_front(3);
        list.push_front(4);
        assert_eq!(0, list.room());

        list.push_back(42);
    }
}
