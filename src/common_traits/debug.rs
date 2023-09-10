use crate::{
    iterator::iter::IterFromFront, node::LinkedListNode, prelude::LinkedList, LinkedListX,
};
use orx_imp_vec::prelude::PinnedVec;
use std::fmt::Debug;

impl<'a, T, P> Debug for LinkedList<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: Debug,
{
    #[allow(clippy::unwrap_in_result)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt(f, self.iter())
    }
}
impl<'a, T, P> Debug for LinkedListX<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: Debug,
{
    #[allow(clippy::unwrap_in_result)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt(f, self.iter())
    }
}

fn fmt<T: Debug>(f: &mut std::fmt::Formatter<'_>, iter: IterFromFront<'_, T>) -> std::fmt::Result {
    write!(
        f,
        "[{}]",
        iter.map(|x| format!("{:?}", x))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug() {
        let mut list = LinkedList::with_exponential_growth(4, 1.6);
        list.push_back('x');
        list.push_back('y');
        list.push_back('z');

        let debug = format!("{:?}", list);
        assert_eq!("['x', 'y', 'z']", debug);

        let debug = format!("{:?}", list.built());
        assert_eq!("['x', 'y', 'z']", debug);
    }
}
