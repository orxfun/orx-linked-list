use crate::{type_aliases::PinVec, Doubly};
use core::iter::FusedIterator;
use orx_selfref_col::{CoreCol, NodePtr};

pub(super) type PairPtr<T> = (NodePtr<Doubly<T>>, NodePtr<Doubly<T>>);

/// An ordered iterator over pointers to the links of the doubly linked list.
///
/// Can be created by calling the `link_iter_ptr` method.
pub struct DoublyLinkIterPtr<'a, T> {
    pub(crate) col: &'a CoreCol<Doubly<T>, PinVec<Doubly<T>>>,
    current: Option<PairPtr<T>>,
    current_back: Option<NodePtr<Doubly<T>>>,
}

impl<'a, T> DoublyLinkIterPtr<'a, T> {
    pub(crate) fn new(
        col: &'a CoreCol<Doubly<T>, PinVec<Doubly<T>>>,
        current: Option<PairPtr<T>>,
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

impl<'a, T> Iterator for DoublyLinkIterPtr<'a, T> {
    type Item = PairPtr<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current {
            Some(p) => {
                let (prev, curr) = p.clone();
                match Some(&curr) == self.current_back.as_ref() {
                    false => {
                        //
                        let next = self.col.node(&curr).next().get();
                        let new_current = next.map(|next| (curr.clone(), next));
                        self.current = new_current;
                    }
                    true => self.end(),
                }

                Some((prev, curr))
            }
            None => None,
        }
    }
}

impl<'a, T> FusedIterator for DoublyLinkIterPtr<'a, T> {}

impl<'a, T> Clone for DoublyLinkIterPtr<'a, T> {
    fn clone(&self) -> Self {
        Self {
            col: self.col,
            current: self.current.clone(),
            current_back: self.current_back.clone(),
        }
    }
}
