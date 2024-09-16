use super::singly_iter_ptr::SinglyIterPtr;
use crate::{type_aliases::PinVec, Singly};
use core::iter::FusedIterator;
use orx_selfref_col::{CoreCol, NodePtr};

/// An ordered iterator over elements of the singly linked list.
///
/// Can be created by calling the `iter` method.
pub struct SinglyIter<'a, T>(SinglyIterPtr<'a, T>);

impl<'a, T> SinglyIter<'a, T> {
    pub(crate) fn new(
        col: &'a CoreCol<Singly<T>, PinVec<Singly<T>>>,
        current: Option<NodePtr<Singly<T>>>,
    ) -> Self {
        Self(SinglyIterPtr::new(col, current))
    }
}

impl<'a, T> Iterator for SinglyIter<'a, T> {
    type Item = &'a T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|p| unsafe { self.0.col.data_unchecked(&p) })
    }
}

impl<'a, T> FusedIterator for SinglyIter<'a, T> {}

impl<'a, T> Clone for SinglyIter<'a, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
