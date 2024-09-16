use crate::{type_aliases::PinVec, Singly};
use core::iter::FusedIterator;
use orx_selfref_col::{CoreCol, NodePtr};

/// An ordered iterator over pointers to the elements of the singly linked list.
///
/// Can be created by calling the `iter_ptr` method.
pub struct SinglyIterPtr<'a, T> {
    pub(crate) col: &'a CoreCol<Singly<T>, PinVec<Singly<T>>>,
    current: Option<NodePtr<Singly<T>>>,
}

impl<'a, T> SinglyIterPtr<'a, T> {
    pub(crate) fn new(
        col: &'a CoreCol<Singly<T>, PinVec<Singly<T>>>,
        current: Option<NodePtr<Singly<T>>>,
    ) -> Self {
        Self { col, current }
    }
}

impl<'a, T> Iterator for SinglyIterPtr<'a, T> {
    type Item = NodePtr<Singly<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current {
            Some(p) => {
                let ptr = Some(p.clone());
                self.current = self.col.node(p).next().get();
                ptr
            }
            None => None,
        }
    }
}

impl<'a, T> FusedIterator for SinglyIterPtr<'a, T> {}

impl<'a, T> Clone for SinglyIterPtr<'a, T> {
    fn clone(&self) -> Self {
        Self {
            col: self.col,
            current: self.current.clone(),
        }
    }
}
