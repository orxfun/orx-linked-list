use crate::{type_aliases::PinVec, Doubly};
use core::iter::FusedIterator;
use orx_selfref_col::{CoreCol, NodePtr};

use super::DoublyIterMut;

/// An ordered iterator mutable references to elements of the doubly linked list.
///
/// Can be created by calling the `iter_mut` method.
pub struct DoublyIterMutChain<'a, T> {
    iter: DoublyIterMut<'a, T>,
    second_front: Option<NodePtr<Doubly<T>>>,
    second_back: Option<NodePtr<Doubly<T>>>,
    consumed_first: bool,
}

impl<'a, T> DoublyIterMutChain<'a, T> {
    pub(crate) fn new(
        col: &'a mut CoreCol<Doubly<T>, PinVec<Doubly<T>>>,
        first: [Option<NodePtr<Doubly<T>>>; 2],
        second: [Option<NodePtr<Doubly<T>>>; 2],
    ) -> Self {
        let iter = DoublyIterMut::new(col, first[0].clone(), first[1].clone());
        let [second_front, second_back] = second;
        Self {
            iter,
            second_front,
            second_back,
            consumed_first: false,
        }
    }
}

impl<'a, T> Iterator for DoublyIterMutChain<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(x) => Some(x),
            None => match self.consumed_first {
                true => None,
                false => {
                    self.consumed_first = true;
                    self.iter
                        .restart_for(self.second_front.clone(), self.second_back.clone());
                    self.iter.next()
                }
            },
        }
    }
}

impl<'a, T> FusedIterator for DoublyIterMutChain<'a, T> {}
