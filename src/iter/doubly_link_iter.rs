use super::{DoublyLinkIterPtr, doubly_link_iter_ptr::PairPtr};
use crate::Doubly;
use core::iter::FusedIterator;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{CoreCol, Node, NodePtr};

/// An ordered iterator over elements of the doubly linked list.
///
/// Can be created by calling the `iter` method.
pub struct DoublyLinkIter<'a, T, P>(DoublyLinkIterPtr<'a, T, P>)
where
    P: PinnedVec<Node<Doubly<T>>>;

impl<'a, T, P> DoublyLinkIter<'a, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    pub(crate) fn new(
        col: &'a CoreCol<Doubly<T>, P>,
        current: Option<PairPtr<T>>,
        current_back: Option<NodePtr<Doubly<T>>>,
    ) -> Self {
        Self(DoublyLinkIterPtr::new(col, current, current_back))
    }
}

impl<'a, T, P> Iterator for DoublyLinkIter<'a, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
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

impl<T, P> FusedIterator for DoublyLinkIter<'_, T, P> where P: PinnedVec<Node<Doubly<T>>> {}

impl<T, P> Clone for DoublyLinkIter<'_, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
