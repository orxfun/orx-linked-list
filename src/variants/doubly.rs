use super::{ends::ListEnds, list_variant::ListVariant};
use crate::list::DefaultMemoryPolicy;
use orx_selfref_col::{
    MemoryReclaimPolicy, Node, NodeDataLazyClose, NodeRefSingle, NodeRefs, NodeRefsArray, Variant,
};
use std::marker::PhantomData;

pub type EndsDoubly<'a, T, M> = NodeRefsArray<'a, 2, Doubly<M>, T>;

impl<'a, T, M> ListEnds<'a, Doubly<M>, T> for EndsDoubly<'a, T, M>
where
    M: 'a + MemoryReclaimPolicy,
{
    fn front(&self) -> Option<&'a Node<'a, Doubly<M>, T>> {
        self.get()[0]
    }

    fn back(&self) -> Option<&'a Node<'a, Doubly<M>, T>> {
        self.get()[1]
    }
}

/// A doubly linked list variant such that:
/// * Each node contains three data elements: the value of the element, a reference to the previous node and a reference to the next node.
/// * The list keeps track of its `front` and `back`.
/// * It is possible to iterate from the `front` to the `back` of the list with `iter` method;
/// and from the `back` to the `front` with `iter_from_back` method.
#[derive(Clone, Copy, Debug)]
pub struct Doubly<M: MemoryReclaimPolicy = DefaultMemoryPolicy> {
    phantom: PhantomData<M>,
}

impl<'a, T, M> Variant<'a, T> for Doubly<M>
where
    T: 'a,
    M: 'a + MemoryReclaimPolicy,
{
    type Storage = NodeDataLazyClose<T>;

    type Prev = NodeRefSingle<'a, Self, T>;

    type Next = NodeRefSingle<'a, Self, T>;

    type Ends = EndsDoubly<'a, T, M>;

    type MemoryReclaim = M;
}

impl<'a, T, M> ListVariant<'a, T> for Doubly<M>
where
    T: 'a,
    M: 'a + MemoryReclaimPolicy,
{
    type PrevNode = NodeRefSingle<'a, Self, T>;

    type NextNode = NodeRefSingle<'a, Self, T>;

    #[cfg(test)]
    fn validate(list: &crate::list::List<'a, Self, T>)
    where
        Self::Ends: ListEnds<'a, Self, T>,
    {
        list.validate_list();
    }
}
