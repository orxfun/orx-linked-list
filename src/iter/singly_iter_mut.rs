use crate::{type_aliases::PinVec, Singly};
use core::iter::FusedIterator;
use orx_selfref_col::{CoreCol, NodePtr};

/// An ordered iterator mutable references to elements of the singly linked list.
///
/// Can be created by calling the `iter_mut` method.
pub struct SinglyIterMut<'a, T> {
    pub(crate) col: &'a mut CoreCol<Singly<T>, PinVec<Singly<T>>>,
    current: Option<NodePtr<Singly<T>>>,
}

impl<'a, T> SinglyIterMut<'a, T> {
    pub(crate) fn new_old(col: &'a mut CoreCol<Singly<T>, PinVec<Singly<T>>>) -> Self {
        let current = col.ends().get();
        Self { col, current }
    }

    pub(crate) fn new(
        col: &'a mut CoreCol<Singly<T>, PinVec<Singly<T>>>,
        current: Option<NodePtr<Singly<T>>>,
    ) -> Self {
        Self { col, current }
    }
}

impl<'a, T> Iterator for SinglyIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current {
            Some(p) => {
                let ptr = p.ptr();
                self.current = self.col.node(p).next().get();
                unsafe { &mut *ptr }.data_mut()
            }
            None => None,
        }
    }
}

impl<'a, T> FusedIterator for SinglyIterMut<'a, T> {}
