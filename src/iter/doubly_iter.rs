use super::doubly_iter_ptr::DoublyIterPtr;
use crate::Doubly;
use core::iter::FusedIterator;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{CoreCol, Node, NodePtr};

/// An ordered iterator over elements of the doubly linked list.
///
/// Can be created by calling the `iter` method.
pub struct DoublyIter<'a, T, P>(DoublyIterPtr<'a, T, P>)
where
    P: PinnedVec<Node<Doubly<T>>>;

impl<'a, T, P> DoublyIter<'a, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    pub(crate) fn new(
        col: &'a CoreCol<Doubly<T>, P>,
        current: Option<NodePtr<Doubly<T>>>,
        current_back: Option<NodePtr<Doubly<T>>>,
    ) -> Self {
        Self(DoublyIterPtr::new(col, current, current_back))
    }
}

impl<'a, T, P> Iterator for DoublyIter<'a, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    type Item = &'a T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|p| unsafe { self.0.col.data_unchecked(p) })
    }
}

impl<T, P> DoubleEndedIterator for DoublyIter<'_, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0
            .next_back()
            .map(|p| unsafe { self.0.col.data_unchecked(p) })
    }
}

impl<T, P> FusedIterator for DoublyIter<'_, T, P> where P: PinnedVec<Node<Doubly<T>>> {}

impl<T, P> Clone for DoublyIter<'_, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
