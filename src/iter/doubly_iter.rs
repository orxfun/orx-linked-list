use super::doubly_iter_ptr::DoublyIterPtr;
use crate::{type_aliases::PinVec, Doubly};
use core::iter::FusedIterator;
use orx_selfref_col::{CoreCol, NodePtr};

/// An ordered iterator over elements of the doubly linked list.
///
/// Can be created by calling the `iter` method.
pub struct DoublyIter<'a, T>(DoublyIterPtr<'a, T>);

impl<'a, T> DoublyIter<'a, T> {
    pub(crate) fn new(
        col: &'a CoreCol<Doubly<T>, PinVec<Doubly<T>>>,
        current: Option<NodePtr<Doubly<T>>>,
        current_back: Option<NodePtr<Doubly<T>>>,
    ) -> Self {
        Self(DoublyIterPtr::new(col, current, current_back))
    }
}

impl<'a, T> Iterator for DoublyIter<'a, T> {
    type Item = &'a T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|p| unsafe { self.0.col.data_unchecked(&p) })
    }
}

impl<'a, T> DoubleEndedIterator for DoublyIter<'a, T> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0
            .next_back()
            .map(|p| unsafe { self.0.col.data_unchecked(&p) })
    }
}

impl<'a, T> FusedIterator for DoublyIter<'a, T> {}

impl<'a, T> Clone for DoublyIter<'a, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
