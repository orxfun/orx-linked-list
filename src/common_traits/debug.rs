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
        write!(
            f,
            "[{}]",
            self.iter()
                .map(|x| format!("{:?}", x))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug() {
        let mut list = LinkedList::with_exponential_growth(4, 1.6, Default::default());
        list.push_back('x');
        list.push_back('y');
        list.push_back('z');

        let debug = format!("{:?}", list);
        assert_eq!("['x', 'y', 'z']", debug);
    }
}
