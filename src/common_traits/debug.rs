use std::fmt::Debug;

use crate::{node::LinkedListNode, prelude::LinkedList};
use orx_imp_vec::prelude::PinnedVec;

impl<'a, T, P> Debug for LinkedList<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: Debug,
{
    #[allow(clippy::unwrap_in_result)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinkedList({}) = [ ", self.len)?;
        let mut curr_node = self.imp[0].prev;
        while let Some(curr) = curr_node {
            write!(f, "{:?} ", curr.data.as_ref().expect("is-some"))?;
            curr_node = curr.next;
        }
        write!(f, "]")
    }
}
