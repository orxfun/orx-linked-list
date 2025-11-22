use crate::Singly;
use core::iter::FusedIterator;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{CoreCol, Node, NodePtr};

/// An ordered consuming iterator of the singly linked list.
///
/// Can be created by calling the `into_iter` method.
pub struct SinglyIterOwned<T, P>
where
    P: PinnedVec<Node<Singly<T>>>,
{
    pub(crate) col: CoreCol<Singly<T>, P>,
    current: Option<NodePtr<Singly<T>>>,
}

impl<T, P> SinglyIterOwned<T, P>
where
    P: PinnedVec<Node<Singly<T>>>,
{
    pub(crate) fn new(col: CoreCol<Singly<T>, P>) -> Self {
        let current = col.ends().get();
        Self { col, current }
    }
}

impl<T, P> Iterator for SinglyIterOwned<T, P>
where
    P: PinnedVec<Node<Singly<T>>>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(p) => {
                // SAFETY: collection as alive as guaranteed by the `col` field.
                let ptr = unsafe { p.ptr_mut() };
                self.current = self.col.node(p).next().get();
                unsafe { &mut *ptr }.take_data()
            }
            None => None,
        }
    }
}

impl<T, P> FusedIterator for SinglyIterOwned<T, P> where P: PinnedVec<Node<Singly<T>>> {}
