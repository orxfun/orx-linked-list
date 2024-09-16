use crate::{type_aliases::PinVec, Doubly};
use core::iter::FusedIterator;
use orx_selfref_col::{CoreCol, NodePtr};

/// An ordered consuming iterator of the doubly linked list.
///
/// Can be created by calling the `into_iter` method.
pub struct DoublyIterOwned<T> {
    col: CoreCol<Doubly<T>, PinVec<Doubly<T>>>,
    current: Option<NodePtr<Doubly<T>>>,
    current_back: Option<NodePtr<Doubly<T>>>,
}

impl<T> DoublyIterOwned<T> {
    pub(crate) fn new(col: CoreCol<Doubly<T>, PinVec<Doubly<T>>>) -> Self {
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

impl<T> Iterator for DoublyIterOwned<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current {
            Some(p) => {
                let ptr = p.ptr();
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

impl<T> DoubleEndedIterator for DoublyIterOwned<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match &self.current_back {
            Some(p) => {
                let ptr = p.ptr();
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

impl<T> FusedIterator for DoublyIterOwned<T> {}
