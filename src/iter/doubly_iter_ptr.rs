use crate::{type_aliases::PinVec, Doubly};
use core::iter::FusedIterator;
use orx_selfref_col::{CoreCol, NodePtr};

/// An ordered iterator over pointers to the elements of the doubly linked list.
///
/// Can be created by calling the `iter_ptr` method.
pub struct DoublyIterPtr<'a, T> {
    pub(crate) col: &'a CoreCol<Doubly<T>, PinVec<Doubly<T>>>,
    current: Option<NodePtr<Doubly<T>>>,
    current_back: Option<NodePtr<Doubly<T>>>,
}

impl<'a, T> DoublyIterPtr<'a, T> {
    pub(crate) fn new(
        col: &'a CoreCol<Doubly<T>, PinVec<Doubly<T>>>,
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

impl<'a, T> Iterator for DoublyIterPtr<'a, T> {
    type Item = NodePtr<Doubly<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current {
            Some(p) => {
                let ptr = Some(p.clone());
                match self.current == self.current_back {
                    false => self.current = self.col.node(p).next().get(),
                    true => self.end(),
                }

                ptr
            }
            None => None,
        }
    }
}

impl<'a, T> DoubleEndedIterator for DoublyIterPtr<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match &self.current_back {
            Some(p) => {
                let ptr = Some(p.clone());

                match self.current == self.current_back {
                    false => self.current_back = self.col.node(p).prev().get(),
                    true => self.end(),
                }

                ptr
            }
            None => None,
        }
    }
}

impl<'a, T> FusedIterator for DoublyIterPtr<'a, T> {}

impl<'a, T> Clone for DoublyIterPtr<'a, T> {
    fn clone(&self) -> Self {
        Self {
            col: self.col,
            current: self.current.clone(),
            current_back: self.current_back.clone(),
        }
    }
}
