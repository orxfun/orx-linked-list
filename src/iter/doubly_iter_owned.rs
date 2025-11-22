use crate::Doubly;
use core::iter::FusedIterator;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{CoreCol, Node, NodePtr};

/// An ordered consuming iterator of the doubly linked list.
///
/// Can be created by calling the `into_iter` method.
pub struct DoublyIterOwned<T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    col: CoreCol<Doubly<T>, P>,
    current: Option<NodePtr<Doubly<T>>>,
    current_back: Option<NodePtr<Doubly<T>>>,
}

impl<T, P> DoublyIterOwned<T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    pub(crate) fn new(col: CoreCol<Doubly<T>, P>) -> Self {
        let current = col.ends().get(0);
        let current_back = col.ends().get(1);
        Self {
            col,
            current,
            current_back,
        }
    }

    fn end(&mut self) {
        self.current = None;
        self.current_back = None;
    }
}

impl<T, P> Iterator for DoublyIterOwned<T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(p) => {
                let ptr = unsafe { p.ptr_mut() };
                match self.current == self.current_back {
                    false => self.current = self.col.node(p).next().get(),
                    true => self.end(),
                }

                unsafe { &mut *ptr }.take_data()
            }
            None => None,
        }
    }
}

impl<T, P> DoubleEndedIterator for DoublyIterOwned<T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.current_back {
            Some(p) => {
                // SAFETY: collection as alive as guaranteed by the `col` field.
                let ptr = unsafe { p.ptr_mut() };
                match self.current == self.current_back {
                    false => self.current_back = self.col.node(p).prev().get(),
                    true => self.end(),
                }

                unsafe { &mut *ptr }.take_data()
            }
            None => None,
        }
    }
}

impl<T, P> FusedIterator for DoublyIterOwned<T, P> where P: PinnedVec<Node<Doubly<T>>> {}
