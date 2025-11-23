use super::DoublyIterMut;
use crate::Doubly;
use core::iter::FusedIterator;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{CoreCol, Node, NodePtr};

/// An ordered iterator mutable references to elements of the doubly linked list.
///
/// Can be created by calling the `iter_mut` method.
pub struct DoublyIterMutChain<'a, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    iter: DoublyIterMut<'a, T, P>,
    second_front: Option<NodePtr<Doubly<T>>>,
    second_back: Option<NodePtr<Doubly<T>>>,
    consumed_first: bool,
}

impl<'a, T, P> DoublyIterMutChain<'a, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    pub(crate) fn new(
        col: &'a mut CoreCol<Doubly<T>, P>,
        first: [Option<NodePtr<Doubly<T>>>; 2],
        second: [Option<NodePtr<Doubly<T>>>; 2],
    ) -> Self {
        let iter = DoublyIterMut::new(col, first[0], first[1]);
        let [second_front, second_back] = second;
        Self {
            iter,
            second_front,
            second_back,
            consumed_first: false,
        }
    }
}

impl<'a, T, P> Iterator for DoublyIterMutChain<'a, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(x) => Some(x),
            None => match self.consumed_first {
                true => None,
                false => {
                    self.consumed_first = true;
                    self.iter.restart_for(self.second_front, self.second_back);
                    self.iter.next()
                }
            },
        }
    }
}

impl<T, P> FusedIterator for DoublyIterMutChain<'_, T, P> where P: PinnedVec<Node<Doubly<T>>> {}
