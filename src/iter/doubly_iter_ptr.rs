use crate::Doubly;
use core::iter::FusedIterator;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{CoreCol, Node, NodePtr};

/// An ordered iterator over pointers to the elements of the doubly linked list.
///
/// Can be created by calling the `iter_ptr` method.
pub struct DoublyIterPtr<'a, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    pub(crate) col: &'a CoreCol<Doubly<T>, P>,
    current: Option<NodePtr<Doubly<T>>>,
    current_back: Option<NodePtr<Doubly<T>>>,
}

impl<'a, T, P> DoublyIterPtr<'a, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    pub(crate) fn new(
        col: &'a CoreCol<Doubly<T>, P>,
        current: Option<NodePtr<Doubly<T>>>,
        current_back: Option<NodePtr<Doubly<T>>>,
    ) -> Self {
        Self {
            col,
            current,
            current_back,
        }
    }

    pub(crate) fn end(&mut self) {
        self.current = None;
        self.current_back = None;
    }
}

impl<T, P> Iterator for DoublyIterPtr<'_, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    type Item = NodePtr<Doubly<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current {
            Some(p) => {
                let ptr = Some(p.clone());
                match self.current == self.current_back {
                    false => self.current = self.col.node(p).next().get().cloned(),
                    true => self.end(),
                }

                ptr
            }
            None => None,
        }
    }
}

impl<T, P> DoubleEndedIterator for DoublyIterPtr<'_, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        match &self.current_back {
            Some(p) => {
                let ptr = Some(p.clone());

                match self.current == self.current_back {
                    false => self.current_back = self.col.node(p).prev().get().cloned(),
                    true => self.end(),
                }

                ptr
            }
            None => None,
        }
    }
}

impl<T, P> FusedIterator for DoublyIterPtr<'_, T, P> where P: PinnedVec<Node<Doubly<T>>> {}

impl<T, P> Clone for DoublyIterPtr<'_, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    fn clone(&self) -> Self {
        Self {
            col: self.col,
            current: self.current.clone(),
            current_back: self.current_back.clone(),
        }
    }
}
