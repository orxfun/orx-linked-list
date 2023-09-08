use crate::{node::LinkedListNode, prelude::LinkedList};
use orx_imp_vec::prelude::{ImpVec, PinnedVec};

/// `LinkedList` holds all elements close to each other in a `PinnedVec`
/// aiming for better cache locality while using thin references rather
/// than wide pointers and to reduce heap allocations.
///
/// In order to achieve *O(1)* time complexity while avoiding smart pointers,
/// remove and pop operations are able to be `Lazy`.
/// In this case; i.e., when the strategy is set to `MemoryReclaimStrategy::Lazy`:
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
/// * `WithThreshold`: `pop_back`, `pop_front` or `remove` method call
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
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(8)
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
    /// assert_eq!(4, util.num_used_nodes);
    /// assert_eq!(1.00, util.utilization());
    ///
    /// // remove 1 of 4
    /// _ = list.remove_at(2);
    /// let util = list.memory_status();
    /// assert_eq!(3, util.num_active_nodes);
    /// assert_eq!(4, util.num_used_nodes);
    /// assert_eq!(0.75, util.utilization());
    ///
    /// // pop 2 more
    /// _ = list.pop_back();
    /// _ = list.pop_front();
    /// let util = list.memory_status();
    /// assert_eq!(1, util.num_active_nodes);
    /// assert_eq!(4, util.num_used_nodes);
    /// assert_eq!(0.25, util.utilization());
    ///
    /// // remove the last element
    /// _ = list.remove_at(0);
    /// let util = list.memory_status();
    /// assert_eq!(0, util.num_active_nodes);
    /// assert_eq!(4, util.num_used_nodes);
    /// assert_eq!(0.00, util.utilization());
    /// ```
    Lazy,
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
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(8)
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
    /// assert_eq!(4, util.num_used_nodes);
    /// assert_eq!(1.00, util.utilization());
    ///
    /// // remove 1 of 4; utilization remains above the threshold
    /// _ = list.remove_at(2);
    /// let util = list.memory_status();
    /// assert_eq!(3, util.num_active_nodes);
    /// assert_eq!(4, util.num_used_nodes);
    /// assert_eq!(0.75, util.utilization());
    ///
    /// // pop 1 more which would reduce utilization to 0.50
    /// // since it is below the treshold; the memory will be reclaimed immediately
    /// _ = list.pop_back();
    /// let util = list.memory_status();
    /// assert_eq!(2, util.num_active_nodes);
    /// assert_eq!(2, util.num_used_nodes);
    /// assert_eq!(1.00, util.utilization());
    /// ```
    WithThreshold(f32),
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
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_doubling_growth(8)
    ///     .with_memory_utilization(MemoryUtilization::Eager);
    ///
    /// // fill list with 4 elements
    /// list.push_back('a');
    /// list.push_back('b');
    /// list.push_back('c');
    /// list.push_back('d');
    ///
    /// let util = list.memory_status();
    /// assert_eq!(4, util.num_active_nodes);
    /// assert_eq!(4, util.num_used_nodes);
    /// assert_eq!(1.00, util.utilization());
    ///
    /// // remove 1 of 4
    /// _ = list.remove_at(2);
    /// let util = list.memory_status();
    /// assert_eq!(3, util.num_active_nodes);
    /// assert_eq!(3, util.num_used_nodes);
    /// assert_eq!(1.00, util.utilization());
    ///
    /// // pop 2 more
    /// _ = list.pop_back();
    /// _ = list.pop_front();
    /// let util = list.memory_status();
    /// assert_eq!(1, util.num_active_nodes);
    /// assert_eq!(1, util.num_used_nodes);
    /// assert_eq!(1.00, util.utilization());
    ///
    /// // remove the last element
    /// _ = list.remove_at(0);
    /// let util = list.memory_status();
    /// assert_eq!(0, util.num_active_nodes);
    /// assert_eq!(0, util.num_used_nodes);
    /// assert_eq!(1.00, util.utilization());
    /// ```
    Eager,
}
impl MemoryUtilization {
    /// Returns a valid memory utilization with:
    /// * `Eager` if `self` is `Eager` or `WithThreshold(x)` where x > 1.0;
    /// * `Lazy` if `self` is `Lazy` or `WithThreshold(x)` where x < 0.0;
    /// * `WithThreshold(x)` otherwise where x in [0.0, 1.0].
    pub(crate) fn into_valid(self) -> MemoryUtilization {
        if let MemoryUtilization::WithThreshold(threshold) = self {
            if threshold < 0.0 {
                MemoryUtilization::Lazy
            } else if threshold > 1.0 {
                MemoryUtilization::Eager
            } else {
                self
            }
        } else {
            self
        }
    }
}
impl Default for MemoryUtilization {
    fn default() -> Self {
        Self::WithThreshold(0.6)
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
///     * `num_active_nodes == num_used_nodes`
/// * in a situation where each push is followed by a pop,
/// memory utilization is 0.0:
///     * `num_active_nodes == 0`
///     * `num_used_nodes == n`, where `n` is the number of items pushed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryStatus {
    /// Number of active nodes in the linked list which is equal to `len` of the list.
    pub num_active_nodes: usize,
    /// Number of total node capacity used by the underlying data structure to store
    /// the active nodes together with the gaps due to `pop_back`, `pop_front` and
    /// `remove` calls.
    pub num_used_nodes: usize,
}
impl MemoryStatus {
    /// Returns `num_active_nodes / num_used_nodes`
    /// as a measure of utilization of the memory used by the linked list.
    pub fn utilization(&self) -> f32 {
        if self.num_used_nodes == 0 {
            debug_assert_eq!(0, self.num_active_nodes);
            1.0
        } else {
            self.num_active_nodes as f32 / self.num_used_nodes as f32
        }
    }
}

impl<'a, T, P> LinkedList<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: 'a,
{
    /// Returns the utilization of the underlying vector of the linked list.
    ///
    /// `LinkedList` holds all elements close to each other in a `PinnedVec`
    /// aiming for better cache locality while using thin references rather
    /// than wide pointers and to reduce heap allocations.
    ///
    /// In order to achieve *O(1)* time complexity while avoiding smart pointers,
    /// remove and pop operations are able to be `Lazy`.
    /// In this case; i.e., when the strategy is set to `MemoryReclaimStrategy::Lazy`:
    /// every `pop_back`, `pop_front` or `remove` method call leaves a gap in the
    /// underlying vector. Status of utilization of the underlying vector can be
    /// queried using the `memory_status` method and the gaps can completely be
    /// reclaimed by manually calling the `memory_reclaim` method which has a time
    /// complexity of *O(n)* where *n* is the length of the underlying vector.
    ///
    /// This method reveals the memory utilization of the underlying pinned vector
    /// at any given time as the fraction of active linked list nodes to total
    /// spaces used by the pinned vector.
    ///
    /// Some extreme examples are as follows:
    ///
    /// * in an push-only situation, memory utilization is equal to 1.0:
    ///     * `num_active_nodes == num_used_nodes`
    /// * in a situation where each push is followed by a pop,
    /// memory utilization is 0.0:
    ///     * `num_active_nodes == 0`
    ///     * `num_used_nodes == n`, where `n` is the number of items pushed.
    ///
    /// # Complexity
    ///
    /// `LinkedList` gives the control over laziness to user:
    ///
    /// * the list can remain lazy throughout the lifetime until it is dropped, or
    /// * at certain points in its lifetime, memory which is not utilized can be
    /// reclaimed by the `memory_reclaim` method.
    ///
    /// `memory_reclaim` shrinks the used memory by placing all linked list elements
    /// next to each other in the underlying pinned vector without leaving any gaps.
    /// This is achieved by a single pass; hence, the method has *O(n)* time complexity.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_exponential_growth(2, 1.5)
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
    /// assert_eq!(4, util.num_used_nodes);
    /// assert_eq!(1.00, util.utilization());
    ///
    /// // remove 1 of 4
    /// _ = list.remove_at(2);
    /// let util = list.memory_status();
    /// assert_eq!(3, util.num_active_nodes);
    /// assert_eq!(4, util.num_used_nodes);
    /// assert_eq!(0.75, util.utilization());
    ///
    /// // pop 2 more
    /// _ = list.pop_back();
    /// _ = list.pop_front();
    /// let util = list.memory_status();
    /// assert_eq!(1, util.num_active_nodes);
    /// assert_eq!(4, util.num_used_nodes);
    /// assert_eq!(0.25, util.utilization());
    ///
    /// // remove the last element
    /// _ = list.remove_at(0);
    /// let util = list.memory_status();
    /// assert_eq!(0, util.num_active_nodes);
    /// assert_eq!(4, util.num_used_nodes);
    /// assert_eq!(0.00, util.utilization());
    /// ```
    pub fn memory_status(&self) -> MemoryStatus {
        MemoryStatus {
            num_active_nodes: self.len,
            num_used_nodes: self.imp.len() - 1,
        }
    }
    /// This method reclaims the gaps which are opened due to lazy pops and removals,
    /// and brings back `memory_status` to 100% in *O(n)* time complexity.
    ///
    /// ```rust ignore
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::default();
    /// list.memory_utilization = MemoryUtilization::Lazy;
    ///
    /// // ...
    /// // regardless of the sequence of pushes, pops, removals
    /// // memory utilization will be 100% after memory_reclaim call.
    /// let _reclaimed = list.memory_reclaim();
    /// assert_eq!(1.00, list.memory_status().utilization());
    /// ```
    ///
    /// `LinkedList` holds all elements close to each other in a `PinnedVec`
    /// aiming for better cache locality while using thin references rather
    /// than wide pointers and to reduce heap allocations.
    ///
    /// In order to achieve *O(1)* time complexity while avoiding smart pointers,
    /// remove and pop operations are able to be `Lazy`.
    /// In this case; i.e., when the strategy is set to `MemoryReclaimStrategy::Lazy`:
    /// every `pop_back`, `pop_front` or `remove` method call leaves a gap in the
    /// underlying vector. Status of utilization of the underlying vector can be
    /// queried using the `memory_status` method and the gaps can completely be
    /// reclaimed by manually calling the `memory_reclaim` method which has a time
    /// complexity of *O(n)* where *n* is the length of the underlying vector.
    ///
    /// In addition to the automatic memory utilization strategy,
    /// memory can be manually reclaimed by using this method.
    /// The `memory_status` method helps here by revealing the memory utilization
    /// at any given time as the fraction of active linked list nodes to total
    /// spaces used by the pinned vector.
    ///
    /// Some extreme examples are as follows:
    ///
    /// * in an push-only situation, memory utilization is equal to 1.0:
    ///     * `num_active_nodes == num_used_nodes`
    /// * in a situation where each push is followed by a pop,
    /// memory utilization is 0.0:
    ///     * `num_active_nodes == 0`
    ///     * `num_used_nodes == n`, where `n` is the number of items pushed.
    ///
    /// # Complexity
    ///
    /// `LinkedList` gives the control over laziness to user:
    ///
    /// * the list can remain lazy throughout the lifetime until it is dropped, or
    /// * at certain points in its lifetime, memory which is not utilized can be
    /// reclaimed by the `memory_reclaim` method having *O(n)* time complexity.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_exponential_growth(2, 1.5)
    ///     .with_memory_utilization(MemoryUtilization::Lazy);
    ///
    /// // build list: c <-> b <-> a <-> d
    /// list.push_back('a');
    /// list.push_front('b');
    /// list.push_front('c');
    /// list.push_back('d');
    ///
    /// assert_eq!(
    ///     list.iter().cloned().collect::<Vec<_>>(),
    ///     ['c', 'b', 'a', 'd'],
    /// );
    /// assert_eq!(1.00, list.memory_status().utilization());
    ///
    /// // nothing to reclaim
    /// let reclaimed = list.memory_reclaim();
    /// assert_eq!(0, reclaimed);
    ///
    /// let popped = list.pop_back();
    /// assert_eq!(Some('d'), popped);
    /// assert_eq!(list.iter().cloned().collect::<Vec<_>>(), ['c', 'b', 'a']);
    /// assert_eq!(0.75, list.memory_status().utilization());
    ///
    /// // one position to reclaim
    /// let reclaimed = list.memory_reclaim();
    /// assert_eq!(1, reclaimed);
    /// assert_eq!(list.iter().cloned().collect::<Vec<_>>(), ['c', 'b', 'a']);
    /// assert_eq!(1.00, list.memory_status().utilization());
    ///
    /// let removed = list.remove_at(1);
    /// assert_eq!('b', removed);
    /// let popped = list.pop_front();
    /// assert_eq!(Some('c'), popped);
    /// assert_eq!(list.iter().cloned().collect::<Vec<_>>(), ['a']);
    /// assert_eq!(1.0 / 3.0, list.memory_status().utilization());
    ///
    /// // two positions to reclaim
    /// let reclaimed = list.memory_reclaim();
    /// assert_eq!(2, reclaimed);
    /// assert_eq!(list.iter().cloned().collect::<Vec<_>>(), ['a']);
    /// assert_eq!(1.00, list.memory_status().utilization());
    ///
    /// // pushing more using reclaimed capacity under the hood
    /// list.push_back('x');
    /// list.push_back('y');
    /// list.push_back('z');
    /// assert_eq!(1.00, list.memory_status().utilization());
    ///
    /// // as one could expect, `clear` does not leave gaps
    /// list.clear();
    /// assert!(list.is_empty());
    /// assert_eq!(1.00, list.memory_status().utilization());
    pub fn memory_reclaim(&mut self) -> usize {
        let mut last_occupied_idx = 0;

        for i in 1..self.imp.len() {
            if self.imp[i].data.is_none() {
                let vacant_idx = i;
                let occupied_idx = Self::get_first_occupied(&self.imp, vacant_idx + 1);

                if let Some(occupied_idx) = occupied_idx {
                    last_occupied_idx = vacant_idx;

                    // update occupied's prev & next
                    let prev_idx = self.node_ind(self.imp[occupied_idx].prev);
                    if let Some(prev_idx) = prev_idx {
                        self.imp.set_next(prev_idx, Some(vacant_idx));
                    } else {
                        // no prev -> front
                        self.set_front(Some(vacant_idx));
                    }

                    let next_idx = self.node_ind(self.imp[occupied_idx].next);
                    if let Some(next_idx) = next_idx {
                        self.imp.set_prev(next_idx, Some(vacant_idx));
                    } else {
                        // no next -> back
                        self.set_back(Some(vacant_idx));
                    }

                    // write to vacant from occupied
                    unsafe { self.imp.unsafe_swap(vacant_idx, occupied_idx) };
                } else {
                    break;
                }
            } else {
                last_occupied_idx = i;
            }
        }

        let to_be_reclaimed = self.imp.len() - 1 - last_occupied_idx;
        dbg!(last_occupied_idx, to_be_reclaimed, self.imp.len());
        unsafe { self.imp.unsafe_truncate(last_occupied_idx + 1) };
        to_be_reclaimed
    }
    fn get_first_occupied(imp: &ImpVec<LinkedListNode<'a, T>, P>, start: usize) -> Option<usize> {
        (start..imp.len()).find(|&i| imp[i].data.is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_eq_f32(first: f32, second: f32) {
        assert!((first - second).abs() < f32::EPSILON);
    }

    #[test]
    fn into_valid() {
        let lazy = MemoryUtilization::Lazy.into_valid();
        assert_eq!(lazy, MemoryUtilization::Lazy);

        let lazy = MemoryUtilization::WithThreshold(-1.0).into_valid();
        assert_eq!(lazy, MemoryUtilization::Lazy);

        let eager = MemoryUtilization::Eager.into_valid();
        assert_eq!(eager, MemoryUtilization::Eager);

        let eager = MemoryUtilization::WithThreshold(3.0).into_valid();
        assert_eq!(eager, MemoryUtilization::Eager);

        let with_threshold = MemoryUtilization::WithThreshold(0.6).into_valid();
        assert_eq!(with_threshold, MemoryUtilization::WithThreshold(0.6));
    }

    #[test]
    fn default_memory_utilization() {
        let with_threshold = MemoryUtilization::default();
        assert_eq!(with_threshold, MemoryUtilization::WithThreshold(0.6));
        assert_eq!(
            with_threshold.into_valid(),
            MemoryUtilization::WithThreshold(0.6)
        );
    }

    #[test]
    fn utilization() {
        let status = MemoryStatus {
            num_active_nodes: 3,
            num_used_nodes: 4,
        };
        assert_eq_f32(0.75, status.utilization());

        let status = MemoryStatus {
            num_active_nodes: 0,
            num_used_nodes: 0,
        };
        assert_eq_f32(1.0, status.utilization());
    }

    #[test]
    fn list_utilization() {
        let mut list =
            LinkedList::with_doubling_growth(4).with_memory_utilization(MemoryUtilization::Lazy);

        assert_eq_f32(1.0, list.memory_status().utilization());

        list.push_back('a');
        assert_eq_f32(1.0, list.memory_status().utilization());

        list.push_back('b');
        assert_eq_f32(1.0, list.memory_status().utilization());

        list.push_back('c');
        assert_eq_f32(1.0, list.memory_status().utilization());

        list.push_back('d');
        assert_eq_f32(1.0, list.memory_status().utilization());

        _ = list.pop_back();
        assert_eq_f32(0.75, list.memory_status().utilization());

        _ = list.pop_front();
        assert_eq_f32(0.50, list.memory_status().utilization());

        _ = list.pop_back();
        assert_eq_f32(0.25, list.memory_status().utilization());

        _ = list.pop_front();
        assert_eq_f32(0.0, list.memory_status().utilization());

        _ = list.pop_front();
        assert_eq_f32(0.0, list.memory_status().utilization());

        list.memory_reclaim();
        assert_eq_f32(1.0, list.memory_status().utilization());

        list.push_back('a');
        assert_eq_f32(1.0, list.memory_status().utilization());

        list.push_back('b');
        assert_eq_f32(1.0, list.memory_status().utilization());

        _ = list.pop_front();
        assert_eq_f32(0.50, list.memory_status().utilization());

        list.memory_reclaim();
        assert_eq_f32(1.0, list.memory_status().utilization());
    }
}
