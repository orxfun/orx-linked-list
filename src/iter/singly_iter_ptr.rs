use crate::Singly;
use core::iter::FusedIterator;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{CoreCol, Node, NodePtr};

/// An ordered iterator over pointers to the elements of the singly linked list.
///
/// Can be created by calling the `iter_ptr` method.
pub struct SinglyIterPtr<'a, T, P>
where
    P: PinnedVec<Node<Singly<T>>>,
{
    pub(crate) col: &'a CoreCol<Singly<T>, P>,
    current: Option<NodePtr<Singly<T>>>,
}

impl<'a, T, P> SinglyIterPtr<'a, T, P>
where
    P: PinnedVec<Node<Singly<T>>>,
{
    pub(crate) fn new(col: &'a CoreCol<Singly<T>, P>, current: Option<NodePtr<Singly<T>>>) -> Self {
        Self { col, current }
    }
}

impl<T, P> Iterator for SinglyIterPtr<'_, T, P>
where
    P: PinnedVec<Node<Singly<T>>>,
{
    type Item = NodePtr<Singly<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(p) => {
                let ptr = Some(p.clone());
                self.current = self.col.node(p).next().get();
                ptr
            }
            None => None,
        }
    }
}

impl<T, P> FusedIterator for SinglyIterPtr<'_, T, P> where P: PinnedVec<Node<Singly<T>>> {}

impl<T, P> Clone for SinglyIterPtr<'_, T, P>
where
    P: PinnedVec<Node<Singly<T>>>,
{
    fn clone(&self) -> Self {
        Self {
            col: self.col,
            current: self.current.clone(),
        }
    }
}
