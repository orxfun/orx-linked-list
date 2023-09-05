use orx_imp_vec::prelude::PinnedVec;

use crate::{node::LinkedListNode, LinkedList};

impl<'a, T, P> LinkedList<'a, T, P>
where
    T: Clone,
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
{
    /// Clones and collects values in the linked list into a standard vector.
    ///
    /// `self.collect_vec()` is simply a shorthand for `self.iter().cloned().collect()`.
    pub fn collect_vec(&self) -> Vec<T> {
        self.iter().cloned().collect()
    }
}
