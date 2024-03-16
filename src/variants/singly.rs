use super::{ends::ListEnds, list_variant::ListVariant};
use crate::list::DefaultMemoryPolicy;
use orx_selfref_col::{
    MemoryReclaimPolicy, Node, NodeDataLazyClose, NodeRefNone, NodeRefSingle, NodeRefs,
    NodeRefsArray, Variant,
};
use std::marker::PhantomData;

pub type EndsSingly<'a, T, M> = NodeRefsArray<'a, 2, Singly<M>, T>;

impl<'a, T, M> ListEnds<'a, Singly<M>, T> for EndsSingly<'a, T, M>
where
    M: MemoryReclaimPolicy,
{
    fn front(&self) -> Option<&'a Node<'a, Singly<M>, T>> {
        self.get()[0]
    }

    fn back(&self) -> Option<&'a Node<'a, Singly<M>, T>> {
        self.get()[1]
    }
}

/// A singly linked list variant such that:
/// * Each node contains two data elements: the value of the element and a reference to the next node.
/// * The list keeps track of its `front`.
/// * It is possible to iterate from the `front` to the back of the list.
#[derive(Clone, Copy, Debug)]
pub struct Singly<M = DefaultMemoryPolicy>
where
    M: MemoryReclaimPolicy,
{
    phantom: PhantomData<M>,
}

impl<'a, T, M> Variant<'a, T> for Singly<M>
where
    T: 'a,
    M: 'a + MemoryReclaimPolicy,
{
    type Storage = NodeDataLazyClose<T>;

    type Prev = NodeRefNone;

    type Next = NodeRefSingle<'a, Self, T>;

    type Ends = EndsSingly<'a, T, M>;

    type MemoryReclaim = M;
}

impl<'a, T, M> ListVariant<'a, T> for Singly<M>
where
    T: 'a,
    M: 'a + MemoryReclaimPolicy,
{
    type PrevNode = NodeRefNone;

    type NextNode = NodeRefSingle<'a, Self, T>;

    #[cfg(test)]
    fn validate(list: &crate::list::List<'a, Self, T>)
    where
        Self::Ends: crate::variants::ends::ListEnds<'a, Self, T>,
    {
        list.validate_list();
    }
}
