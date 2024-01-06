use crate::linked_list::LinkedList;

/// `LinkedList` holds all elements close to each other in a `PinnedVec`
/// aiming for better cache locality while using thin references rather
/// than wide pointers and to reduce heap allocations.
///
/// In order to achieve *O(1)* time complexity while avoiding smart pointers,
/// remove and pop operations are able to be `Lazy`.
/// In this case; i.e., when the strategy is set to `MemoryUtilization::Lazy`,
/// every `pop_back`, `pop_front` or `remove` method call leaves a gap in the
/// underlying vector. Status of utilization of the underlying vector can be
/// queried using the `memory_status` method and the gaps can completely be
/// reclaimed by manually calling the `memory_reclaim` method which has a time
/// complexity of *O(n)* where *n* is the length of the underlying vector.
///
/// Being able to be lazy, it is possible to define and use different
/// strategies which would be a better fit for different situations:
///
/// * `Lazy`: `memory_reclaim` is never called automatically:
///     * leads to the cheapest possible `pop_back`, `pop_front` or `remove` operations,
///     * however, the utilization of the vector can be low especially when
/// a large number of elements enter and exit the linked list.
///     * might be a better fit where keeping the time complexity of these operations
/// at *O(1)* is important; or when utilization is not expected to drop too low.
/// * `Eager`: every `pop_back`, `pop_front` or `remove` method call is followed
/// by a `memory_reclaim` call:
///     * this strategy keeps the vector without gaps at 100% utilization;
///     * however, abovementioned operations require *O(n)* time complexity;
///     * might be a better fit where memory is scarce and more important than
/// the increased time-complexity of these methods.
/// * `WithThreshold` (**recommended & default**): `pop_back`, `pop_front` or `remove` method call
/// is followed by a `memory_reclaim` call only if the memory utilization drops below a
/// pre-determined threshold:
///     * this strategy is a generalization of `Lazy` and `Eager` allowing to
/// select the required threshold level between memory utilization and amortized
/// time complexity of these methods. Note that setting the least memory utilization
/// to a value lower than 1.0 would still least to a constant amortized time complexity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryUtilization {
    /// With `Lazy` strategy, `memory_reclaim` is never called automatically:
    /// * leads to the cheapest possible `pop_back`, `pop_front` or `remove_at` operations,
    /// * however, the utilization of the vector can be low especially when
    /// a large number of elements enter and exit the linked list.
    /// * might be a better fit where keeping the time complexity of these operations
    /// at *O(1)* is important; or when utilization is not expected to drop too low.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new()
    ///     .with_memory_utilization(MemoryUtilization::Lazy);
    ///
    /// // fill list with 4 elements
    /// list.push_back('a');
    /// list.push_back('b');
    /// list.push_back('c');
    /// list.push_back('d');
    ///
    /// let util = list.memory_status();
    /// assert_eq!(4, util.num_active_nodes);
    /// assert_eq!(4, util.num_occupied_nodes);
    /// assert_eq!(1.00, util.utilization());
    ///
    /// // remove 1 of 4
    /// _ = list.remove_at(2);
    /// let util = list.memory_status();
    /// assert_eq!(3, util.num_active_nodes);
    /// assert_eq!(4, util.num_occupied_nodes);
    /// assert_eq!(0.75, util.utilization());
    ///
    /// // pop 2 more
    /// _ = list.pop_back();
    /// _ = list.pop_front();
    /// let util = list.memory_status();
    /// assert_eq!(1, util.num_active_nodes);
    /// assert_eq!(4, util.num_occupied_nodes);
    /// assert_eq!(0.25, util.utilization());
    ///
    /// // remove the last element
    /// _ = list.remove_at(0);
    /// let util = list.memory_status();
    /// assert_eq!(0, util.num_active_nodes);
    /// assert_eq!(4, util.num_occupied_nodes);
    /// assert_eq!(0.00, util.utilization());
    /// ```
    Lazy,
    /// With `Eager` strategy, every `pop_back`, `pop_front` or `remove_at` method call is followed
    /// by a `memory_reclaim` call:
    /// * this strategy keeps the vector without gaps at 100% utilization;
    /// * however, abovementioned operations require *O(n)* time complexity;
    /// * might be a better fit where memory is scarce and more important than
    /// the increased time-complexity of these methods.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new().with_memory_utilization(MemoryUtilization::Eager);
    ///
    /// // fill list with 4 elements
    /// list.push_back('a');
    /// list.push_back('b');
    /// list.push_back('c');
    /// list.push_back('d');
    ///
    /// let util = list.memory_status();
    /// assert_eq!(4, util.num_active_nodes);
    /// assert_eq!(4, util.num_occupied_nodes);
    /// assert_eq!(1.00, util.utilization());
    ///
    /// // remove 1 of 4
    /// _ = list.remove_at(2);
    /// let util = list.memory_status();
    /// assert_eq!(3, util.num_active_nodes);
    /// assert_eq!(3, util.num_occupied_nodes);
    /// assert_eq!(1.00, util.utilization());
    ///
    /// // pop 2 more
    /// _ = list.pop_back();
    /// _ = list.pop_front();
    /// let util = list.memory_status();
    /// assert_eq!(1, util.num_active_nodes);
    /// assert_eq!(1, util.num_occupied_nodes);
    /// assert_eq!(1.00, util.utilization());
    ///
    /// // remove the last element
    /// _ = list.remove_at(0);
    /// let util = list.memory_status();
    /// assert_eq!(0, util.num_active_nodes);
    /// assert_eq!(0, util.num_occupied_nodes);
    /// assert_eq!(1.00, util.utilization());
    /// ```
    Eager,
    /// With `WithThreshold`strategy, `pop_back`, `pop_front` or `remove_at` method call
    /// is followed by a `memory_reclaim` call only if the memory utilization drops below the
    /// pre-determined threshold:
    ///     * this strategy is a generalization of `Lazy` and `Eager` allowing to
    /// select the required threshold level between memory utilization and amortized
    /// time complexity of these methods. Note that setting the least memory utilization
    /// to a value lower than 1.0 would still least to a constant amortized time complexity.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::*;
    ///
    /// let mut list = LinkedList::new()
    ///     .with_memory_utilization(MemoryUtilization::WithThreshold(0.51));
    ///
    /// // fill list with 4 elements
    /// list.push_back('a');
    /// list.push_back('b');
    /// list.push_back('c');
    /// list.push_back('d');
    ///
    /// let util = list.memory_status();
    /// assert_eq!(4, util.num_active_nodes);
    /// assert_eq!(4, util.num_occupied_nodes);
    /// assert_eq!(1.00, util.utilization());
    ///
    /// // remove 1 of 4; utilization remains above the threshold
    /// _ = list.remove_at(2);
    /// let util = list.memory_status();
    /// assert_eq!(3, util.num_active_nodes);
    /// assert_eq!(4, util.num_occupied_nodes);
    /// assert_eq!(0.75, util.utilization());
    ///
    /// // pop 1 more which would reduce utilization to 0.50
    /// // since it is below the treshold; the memory will be reclaimed immediately
    /// _ = list.pop_back();
    /// let util = list.memory_status();
    /// assert_eq!(2, util.num_active_nodes);
    /// assert_eq!(2, util.num_occupied_nodes);
    /// assert_eq!(1.00, util.utilization());
    /// ```
    WithThreshold(f32),
}

impl Default for MemoryUtilization {
    fn default() -> Self {
        Self::WithThreshold(0.75)
    }
}

impl MemoryUtilization {
    #[inline(always)]
    fn will_auto_reclaim(self, list_len: usize, storage_len: usize) -> bool {
        match self {
            MemoryUtilization::Eager => storage_len > list_len,
            MemoryUtilization::Lazy => false,
            MemoryUtilization::WithThreshold(threshold) => {
                storage_len > list_len && {
                    let utilization = list_len as f32 / storage_len as f32;
                    utilization < threshold
                }
            }
        }
    }
    pub(crate) fn reclaim<T>(self, list: &mut LinkedList<'_, T>, storage_len: usize) {
        if self.will_auto_reclaim(list.len(), storage_len) {
            reclaim_memory(list, storage_len)
        }
    }
}

/// Utilization of the underlying vector of the linked list.
///
/// `LinkedList` holds all elements close to each other in a `PinnedVec`
/// aiming for better cache locality while using thin references rather
/// than wide pointers and to reduce heap allocations.
///
/// In order to achieve *O(1)* time complexity while avoiding smart pointers,
/// remove and pop operations are designed to be lazy:
///
/// * the links are immediately adjusted; however,
/// * the memory is not immediately reclaimed leaving a gap in the underlying vector.
///
/// This method reveals the memory utilization of the underlying pinned vector
/// at any given time as the fraction of active linked list nodes to total
/// spaces used by the pinned vector.
///
/// Some extreme examples are as follows:
///
/// * in an push-only situation, memory utilization is equal to 1.0:
///     * `num_active_nodes == num_occupied_nodes`
/// * in a situation where each push is followed by a pop,
/// memory utilization is 0.0:
///     * `num_active_nodes == 0`
///     * `num_occupied_nodes == n`, where `n` is the number of items pushed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryStatus {
    /// Number of active nodes in the linked list which is equal to `len` of the list.
    pub num_active_nodes: usize,
    /// Number of total node capacity used by the underlying data structure to store
    /// the active nodes together with the gaps due to `pop_back`, `pop_front` and
    /// `remove` calls.
    pub num_occupied_nodes: usize,
}

impl MemoryStatus {
    /// Returns `num_active_nodes / num_occupied_nodes` as a measure of utilization of the memory used by the linked list.
    pub fn utilization(&self) -> f32 {
        if self.num_occupied_nodes == 0 {
            debug_assert_eq!(0, self.num_active_nodes);
            1.0
        } else {
            self.num_active_nodes as f32 / self.num_occupied_nodes as f32
        }
    }
    pub(crate) fn of_list(list_len: usize, storage_len: usize) -> Self {
        let num_active_nodes = list_len;
        let num_occupied_nodes = storage_len;
        Self {
            num_active_nodes,
            num_occupied_nodes,
        }
    }
}

pub(crate) fn reclaim_memory<T>(list: &mut LinkedList<'_, T>, storage_len: usize) {
    let mut last_occupied_idx = storage_len;

    for i in 0..storage_len {
        if list.is_vacant(i) {
            let vacant_idx = i;
            let occupied_idx = get_last_occupied_idx(list, vacant_idx + 1, last_occupied_idx);
            if let Some(occupied_idx) = occupied_idx {
                last_occupied_idx = occupied_idx;
                list.move_to_vacant_node(occupied_idx, vacant_idx);
            } else {
                break;
            }
        }
    }

    list.truncate_vec();
}

fn get_last_occupied_idx<T>(list: &LinkedList<'_, T>, start: usize, end: usize) -> Option<usize> {
    (start..end).rev().find(|&i| !list.is_vacant(i))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::linked_list::{tests::storage_to_datavec, LinkedList};
    use std::fmt::Debug;
    type List<'a, T> = LinkedList<'a, T>;

    fn status_of<T>(storage: &[Option<T>]) -> MemoryStatus {
        MemoryStatus {
            num_active_nodes: storage.iter().filter(|x| x.is_some()).count(),
            num_occupied_nodes: storage.len(),
        }
    }
    fn assert_storage<T>(list: &List<T>, expected_storage: &[Option<T>])
    where
        T: Debug + Clone + PartialEq,
    {
        assert_eq!(expected_storage, storage_to_datavec(list).as_slice());
        assert_eq!(status_of(expected_storage), list.memory_status());
    }

    #[test]
    fn will_auto_reclaim() {
        let eager = MemoryUtilization::Eager;
        assert!(!eager.will_auto_reclaim(0, 0));
        assert!(!eager.will_auto_reclaim(10, 10));
        assert!(eager.will_auto_reclaim(0, 1));
        assert!(eager.will_auto_reclaim(10, 11));

        let lazy = MemoryUtilization::Lazy;
        assert!(!lazy.will_auto_reclaim(0, 0));
        assert!(!lazy.will_auto_reclaim(10, 10));
        assert!(!lazy.will_auto_reclaim(0, 1));
        assert!(!lazy.will_auto_reclaim(10, 11));

        let threashold_eager = MemoryUtilization::WithThreshold(1.0);
        assert!(!threashold_eager.will_auto_reclaim(0, 0));
        assert!(!threashold_eager.will_auto_reclaim(10, 10));
        assert!(threashold_eager.will_auto_reclaim(0, 1));
        assert!(threashold_eager.will_auto_reclaim(10, 11));

        let threashold_lazy = MemoryUtilization::WithThreshold(0.0);
        assert!(!threashold_lazy.will_auto_reclaim(0, 0));
        assert!(!threashold_lazy.will_auto_reclaim(10, 10));
        assert!(!threashold_lazy.will_auto_reclaim(0, 1));
        assert!(!threashold_lazy.will_auto_reclaim(10, 11));

        let threashold_eager = MemoryUtilization::WithThreshold(0.25);
        assert!(!threashold_eager.will_auto_reclaim(0, 0));
        assert!(!threashold_eager.will_auto_reclaim(10, 10));
        assert!(threashold_eager.will_auto_reclaim(0, 1));
        assert!(!threashold_eager.will_auto_reclaim(5, 16));
        assert!(!threashold_eager.will_auto_reclaim(4, 16));
        assert!(threashold_eager.will_auto_reclaim(3, 16));

        let threashold_eager = MemoryUtilization::WithThreshold(0.75);
        assert!(!threashold_eager.will_auto_reclaim(0, 0));
        assert!(!threashold_eager.will_auto_reclaim(10, 10));
        assert!(threashold_eager.will_auto_reclaim(0, 1));
        assert!(!threashold_eager.will_auto_reclaim(13, 16));
        assert!(!threashold_eager.will_auto_reclaim(12, 16));
        assert!(threashold_eager.will_auto_reclaim(11, 16));
    }

    #[test]
    fn reclaim_when_no_holes() {
        let mut list = List::new().with_memory_utilization(MemoryUtilization::Lazy);
        list.push_back('a');
        list.push_back('b');
        list.push_back('c');

        assert_storage(&list, &[Some('a'), Some('b'), Some('c')]);

        reclaim_memory(&mut list, 3);

        assert_storage(&list, &[Some('a'), Some('b'), Some('c')]);
    }

    #[test]
    fn reclaim_after_pop_back() {
        let mut list = List::new().with_memory_utilization(MemoryUtilization::Lazy);
        list.push_back('a');
        list.push_back('b');
        list.push_back('c');

        assert_storage(&list, &[Some('a'), Some('b'), Some('c')]);

        list.push_back('d');
        _ = list.pop_back();
        assert_storage(&list, &[Some('a'), Some('b'), Some('c'), None]);

        reclaim_memory(&mut list, 4);
        assert_storage(&list, &[Some('a'), Some('b'), Some('c')]);

        list.push_back('d');
        list.push_back('e');
        _ = list.pop_back();
        _ = list.pop_back();
        assert_storage(&list, &[Some('a'), Some('b'), Some('c'), None, None]);

        reclaim_memory(&mut list, 5);
        assert_storage(&list, &[Some('a'), Some('b'), Some('c')]);
    }

    #[test]
    fn reclaim_after_pop_front() {
        let mut list = List::new().with_memory_utilization(MemoryUtilization::Lazy);
        list.push_back('a');
        list.push_back('b');
        list.push_back('c');

        _ = list.pop_front();
        assert_storage(&list, &[None, Some('b'), Some('c')]);

        reclaim_memory(&mut list, 3);
        assert_storage(&list, &[Some('c'), Some('b')]);

        reclaim_memory(&mut list, 2);
        assert_storage(&list, &[Some('c'), Some('b')]);

        let mut list = List::new().with_memory_utilization(MemoryUtilization::Lazy);
        list.push_back('a');
        list.push_back('b');
        list.push_back('c');

        _ = list.pop_front();
        _ = list.pop_front();
        assert_storage(&list, &[None, None, Some('c')]);

        reclaim_memory(&mut list, 3);
        assert_storage(&list, &[Some('c')]);

        reclaim_memory(&mut list, 1);
        assert_storage(&list, &[Some('c')]);
    }

    #[test]
    fn reclaim_after_pop_back_front() {
        let mut list = List::new().with_memory_utilization(MemoryUtilization::Lazy);
        list.push_back('a');
        list.push_back('b');
        list.push_back('c');

        _ = list.pop_front();
        assert_storage(&list, &[None, Some('b'), Some('c')]);

        _ = list.pop_back();
        assert_storage(&list, &[None, Some('b'), None]);

        reclaim_memory(&mut list, 3);
        assert_storage(&list, &[Some('b')]);

        reclaim_memory(&mut list, 1);
        assert_storage(&list, &[Some('b')]);
    }

    #[test]
    fn reclaim_after_remove() {
        let mut list = List::new().with_memory_utilization(MemoryUtilization::Lazy);
        list.push_back('a');
        list.push_back('b');
        list.push_back('c');

        _ = list.remove_at(1);
        assert_storage(&list, &[Some('a'), None, Some('c')]);

        reclaim_memory(&mut list, 3);
        assert_storage(&list, &[Some('a'), Some('c')]);

        let mut list = List::new().with_memory_utilization(MemoryUtilization::Lazy);
        list.push_back('a');
        list.push_back('b');
        list.push_back('c');
        list.push_back('d');
        list.push_back('e');

        _ = list.remove_at(3);
        _ = list.remove_at(1);
        assert_storage(&list, &[Some('a'), None, Some('c'), None, Some('e')]);

        reclaim_memory(&mut list, 5);
        assert_storage(&list, &[Some('a'), Some('e'), Some('c')]);
    }
}
