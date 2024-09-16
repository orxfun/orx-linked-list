use crate::{type_aliases::PinVec, Doubly};
use core::iter::FusedIterator;
use orx_selfref_col::{CoreCol, NodePtr};

/// An ordered iterator mutable references to elements of the doubly linked list.
///
/// Can be created by calling the `iter_mut` method.
pub struct DoublyIterMut<'a, T> {
    col: &'a mut CoreCol<Doubly<T>, PinVec<Doubly<T>>>,
    current: Option<NodePtr<Doubly<T>>>,
    current_back: Option<NodePtr<Doubly<T>>>,
}

impl<'a, T> DoublyIterMut<'a, T> {
    pub(crate) fn new(
        col: &'a mut CoreCol<Doubly<T>, PinVec<Doubly<T>>>,
        current: Option<NodePtr<Doubly<T>>>,
        current_back: Option<NodePtr<Doubly<T>>>,
    ) -> Self {
        Self {
            col,
            current,
            current_back,
        }
    }

    pub(crate) fn restart_for(
        &mut self,
        current: Option<NodePtr<Doubly<T>>>,
        current_back: Option<NodePtr<Doubly<T>>>,
    ) {
        self.current = current;
        self.current_back = current_back;
    }

    fn end(&mut self) {
        self.current = None;
        self.current_back = None;
    }
}

impl<'a, T> Iterator for DoublyIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current {
            Some(p) => {
                let ptr = p.ptr();
                match self.current == self.current_back {
                    false => self.current = self.col.node(p).next().get(),
                    true => self.end(),
                }

                unsafe { &mut *ptr }.data_mut()
            }
            None => None,
        }
    }
}

impl<'a, T> DoubleEndedIterator for DoublyIterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match &self.current_back {
            Some(p) => {
                let ptr = p.ptr();
                match self.current == self.current_back {
                    false => self.current_back = self.col.node(p).prev().get(),
                    true => self.end(),
                }
                unsafe { &mut *ptr }.data_mut()
            }
            None => None,
        }
    }
}

impl<'a, T> FusedIterator for DoublyIterMut<'a, T> {}
