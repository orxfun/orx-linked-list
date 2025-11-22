use crate::Singly;
use core::iter::FusedIterator;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{CoreCol, Node, NodePtr};

/// An ordered iterator mutable references to elements of the singly linked list.
///
/// Can be created by calling the `iter_mut` method.
pub struct SinglyIterMut<'a, T, P>
where
    P: PinnedVec<Node<Singly<T>>>,
{
    pub(crate) col: &'a mut CoreCol<Singly<T>, P>,
    current: Option<NodePtr<Singly<T>>>,
}

impl<'a, T, P> SinglyIterMut<'a, T, P>
where
    P: PinnedVec<Node<Singly<T>>>,
{
    pub(crate) fn new_old(col: &'a mut CoreCol<Singly<T>, P>) -> Self {
        let current = col.ends().get().cloned();
        Self { col, current }
    }

    pub(crate) fn new(
        col: &'a mut CoreCol<Singly<T>, P>,
        current: Option<NodePtr<Singly<T>>>,
    ) -> Self {
        Self { col, current }
    }
}

impl<'a, T, P> Iterator for SinglyIterMut<'a, T, P>
where
    P: PinnedVec<Node<Singly<T>>>,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current {
            Some(p) => {
                // SAFETY: collection as alive as guaranteed by the `col` field.
                let ptr = unsafe { p.ptr_mut() };
                self.current = self.col.node(p).next().get().cloned();
                unsafe { &mut *ptr }.data_mut()
            }
            None => None,
        }
    }
}

impl<T, P> FusedIterator for SinglyIterMut<'_, T, P> where P: PinnedVec<Node<Singly<T>>> {}
