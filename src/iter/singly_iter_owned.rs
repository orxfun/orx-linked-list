use crate::{type_aliases::PinVec, Singly};
use core::iter::FusedIterator;
use orx_selfref_col::{CoreCol, NodePtr};

/// An ordered consuming iterator of the singly linked list.
///
/// Can be created by calling the `into_iter` method.
pub struct SinglyIterOwned<T> {
    pub(crate) col: CoreCol<Singly<T>, PinVec<Singly<T>>>,
    current: Option<NodePtr<Singly<T>>>,
}

impl<T> SinglyIterOwned<T> {
    pub(crate) fn new(col: CoreCol<Singly<T>, PinVec<Singly<T>>>) -> Self {
        let current = col.ends().get();
        Self { col, current }
    }
}

impl<T> Iterator for SinglyIterOwned<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current {
            Some(p) => {
                let ptr = p.ptr();
                self.current = self.col.node(p).next().get();
                unsafe { &mut *ptr }.take_data()
            }
            None => None,
        }
    }
}

impl<T> FusedIterator for SinglyIterOwned<T> {}
