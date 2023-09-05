use orx_imp_vec::prelude::PinnedVec;

use crate::{node::LinkedListNode, LinkedList};

impl<'a, T, P> LinkedList<'a, T, P>
where
    T: Clone,
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
{
    pub fn collect_vec(&self) -> Vec<T> {
        self.iter().cloned().collect()
    }
}
