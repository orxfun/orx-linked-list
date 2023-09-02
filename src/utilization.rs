use crate::prelude::{LinkedList, LinkedListNode};
use orx_imp_vec::{prelude::PinnedVec, ImpVec};

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
/// This method reveals the memory utilizaiton of the underlying pinned vector
/// at any given time as the fraction of active linked list nodes to total
/// spaces used by the pinned vector.
///
/// Some extreme examples are as follows:
///
/// * in an push-only situation, memory utilizaiton is equal to 1.0:
///     * `num_active_nodes == num_used_nodes`
/// * in a situation where each push is followed by a pop,
/// memory utilization is 0.0:
///     * `num_active_nodes == 0`
///     * `num_used_nodes == n`, where `n` is the number of items pushed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkedListMemoryUtilization {
    pub num_active_nodes: usize,
    pub num_used_nodes: usize,
}
impl LinkedListMemoryUtilization {
    /// Returns `num_active_nodes / num_used_nodes`
    /// as a measure of utilization of the claimed memory by the linked list.
    ///
    /// Utilization is `1.0` when no memory is claimed by the linked list.
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
    /// remove and pop operations are designed to be lazy:
    ///
    /// * the links are immediately adjusted; however,
    /// * the memory is not immediately reclaimed leaving a gap in the underlying vector.
    ///
    /// This method reveals the memory utilizaiton of the underlying pinned vector
    /// at any given time as the fraction of active linked list nodes to total
    /// spaces used by the pinned vector.
    ///
    /// Some extreme examples are as follows:
    ///
    /// * in an push-only situation, memory utilizaiton is equal to 1.0:
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
    /// let mut list = LinkedList::with_exponential_growth(2, 1.5);
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
    /// _ = list.remove(2);
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
    /// _ = list.remove(0);
    /// let util = list.memory_status();
    /// assert_eq!(0, util.num_active_nodes);
    /// assert_eq!(4, util.num_used_nodes);
    /// assert_eq!(0.00, util.utilization());
    /// ```
    pub fn memory_status(&self) -> LinkedListMemoryUtilization {
        LinkedListMemoryUtilization {
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
    /// remove and pop operations are designed to be lazy:
    ///
    /// * the links are immediately adjusted; however,
    /// * the memory is not immediately reclaimed leaving a gap in the underlying vector.
    ///
    /// The `memory_status` reveals the memory utilizaiton of the underlying pinned vector
    /// at any given time as the fraction of active linked list nodes to total
    /// spaces used by the pinned vector.
    ///
    /// Some extreme examples are as follows:
    ///
    /// * in an push-only situation, memory utilizaiton is equal to 1.0:
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
    /// let mut list = LinkedList::with_exponential_growth(2, 1.5);
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
    /// let removed = list.remove(1);
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
                let occupied_idx = Self::get_first_occupied(&self.imp, vacant_idx + i);

                if let Some(occupied_idx) = occupied_idx {
                    last_occupied_idx = occupied_idx;

                    // update occupied's prev & next
                    let prev_idx = self.imp[occupied_idx].prev.map(|node| node.ind);
                    if let Some(prev_idx) = prev_idx {
                        self.imp.set_next(prev_idx, Some(vacant_idx));
                    } else {
                        // no prev -> front
                        self.set_front(Some(vacant_idx));
                    }

                    let next_idx = self.imp[occupied_idx].next.map(|node| node.ind);
                    if let Some(next_idx) = next_idx {
                        self.imp.set_prev(next_idx, Some(vacant_idx));
                    } else {
                        // no next -> back
                        self.set_back(Some(vacant_idx));
                    }

                    // write to vacant from occupied
                    unsafe { self.imp.unsafe_swap(vacant_idx, occupied_idx) };
                    self.imp[vacant_idx].ind = vacant_idx;
                } else {
                    break;
                }
            } else {
                last_occupied_idx = i;
            }
        }

        let to_be_reclaimed = self.imp.len() - 1 - last_occupied_idx;
        unsafe { self.imp.unsafe_truncate(last_occupied_idx + 1) };
        to_be_reclaimed
    }
    fn get_first_occupied(imp: &ImpVec<LinkedListNode<'a, T>, P>, start: usize) -> Option<usize> {
        (start..imp.len()).find(|&i| imp[i].data.is_some())
    }
}
