use crate::Doubly;
use core::iter::FusedIterator;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{CoreCol, Node, NodePtr};

pub(super) type PairPtr<T> = (NodePtr<Doubly<T>>, NodePtr<Doubly<T>>);

/// An ordered iterator over pointers to the links of the doubly linked list.
///
/// Can be created by calling the `link_iter_ptr` method.
pub struct DoublyLinkIterPtr<'a, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    pub(crate) col: &'a CoreCol<Doubly<T>, P>,
    current: Option<PairPtr<T>>,
    current_back: Option<NodePtr<Doubly<T>>>,
}

impl<'a, T, P> DoublyLinkIterPtr<'a, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    pub(crate) fn new(
        col: &'a CoreCol<Doubly<T>, P>,
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

impl<T, P> Iterator for DoublyLinkIterPtr<'_, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    type Item = PairPtr<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(p) => {
                let (prev, curr) = p;
                match Some(&curr) == self.current_back.as_ref() {
                    false => {
                        let next = self.col.node(curr).next().get();
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

impl<T, P> FusedIterator for DoublyLinkIterPtr<'_, T, P> where P: PinnedVec<Node<Doubly<T>>> {}

impl<T, P> Clone for DoublyLinkIterPtr<'_, T, P>
where
    P: PinnedVec<Node<Doubly<T>>>,
{
    fn clone(&self) -> Self {
        Self {
            col: self.col,
            current: self.current,
            current_back: self.current_back,
        }
    }
}
