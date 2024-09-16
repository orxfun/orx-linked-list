use super::{doubly_link_iter_ptr::PairPtr, DoublyLinkIterPtr};
use crate::{type_aliases::PinVec, Doubly};
use core::iter::FusedIterator;
use orx_selfref_col::{CoreCol, NodePtr};

/// An ordered iterator over elements of the doubly linked list.
///
/// Can be created by calling the `iter` method.
pub struct DoublyLinkIter<'a, T>(DoublyLinkIterPtr<'a, T>);

impl<'a, T> DoublyLinkIter<'a, T> {
    pub(crate) fn new(
        col: &'a CoreCol<Doubly<T>, PinVec<Doubly<T>>>,
        current: Option<PairPtr<T>>,
        current_back: Option<NodePtr<Doubly<T>>>,
    ) -> Self {
        Self(DoublyLinkIterPtr::new(col, current, current_back))
    }
}

impl<'a, T> Iterator for DoublyLinkIter<'a, T> {
    type Item = (&'a T, &'a T);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|p| {
            (unsafe { self.0.col.data_unchecked(&p.0) }, unsafe {
                self.0.col.data_unchecked(&p.1)
            })
        })
    }
}

impl<'a, T> FusedIterator for DoublyLinkIter<'a, T> {}

impl<'a, T> Clone for DoublyLinkIter<'a, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
