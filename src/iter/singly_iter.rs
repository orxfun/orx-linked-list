use super::singly_iter_ptr::SinglyIterPtr;
use crate::Singly;
use core::iter::FusedIterator;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{CoreCol, Node, NodePtr};

/// An ordered iterator over elements of the singly linked list.
///
/// Can be created by calling the `iter` method.
pub struct SinglyIter<'a, T, P>(SinglyIterPtr<'a, T, P>)
where
    P: PinnedVec<Node<Singly<T>>>;

impl<'a, T, P> SinglyIter<'a, T, P>
where
    P: PinnedVec<Node<Singly<T>>>,
{
    pub(crate) fn new(col: &'a CoreCol<Singly<T>, P>, current: Option<NodePtr<Singly<T>>>) -> Self {
        Self(SinglyIterPtr::new(col, current))
    }
}

impl<'a, T, P> Iterator for SinglyIter<'a, T, P>
where
    P: PinnedVec<Node<Singly<T>>>,
{
    type Item = &'a T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|p| unsafe { self.0.col.data_unchecked(&p) })
    }
}

impl<'a, T, P> FusedIterator for SinglyIter<'a, T, P> where P: PinnedVec<Node<Singly<T>>> {}

impl<'a, T, P> Clone for SinglyIter<'a, T, P>
where
    P: PinnedVec<Node<Singly<T>>>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
