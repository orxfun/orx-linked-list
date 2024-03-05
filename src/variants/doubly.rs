use super::{ends::ListEnds, list_variant::ListVariant};
use orx_selfref_col::{
    MemoryReclaimOnThreshold, Node, NodeDataLazyClose, NodeRefSingle, NodeRefs, NodeRefsArray,
    Variant,
};

pub type EndsDoubly<'a, T> = NodeRefsArray<'a, 2, Doubly, T>;

impl<'a, T> ListEnds<'a, Doubly, T> for EndsDoubly<'a, T> {
    fn front(&self) -> Option<&'a Node<'a, Doubly, T>> {
        self.get()[0]
    }

    fn back(&self) -> Option<&'a Node<'a, Doubly, T>> {
        self.get()[1]
    }
}

/// A doubly linked list variant such that:
/// * Each node contains three data elements: the value of the element, a reference to the previous node and a reference to the next node.
/// * The list keeps track of its `front` and `back`.
/// * It is possible to iterate from the `front` to the `back` of the list with `iter` method;
/// and from the `back` to the `front` with `iter_from_back` method.
#[derive(Clone, Copy, Debug)]
pub struct Doubly;

impl<'a, T> Variant<'a, T> for Doubly
where
    T: 'a,
{
    type Storage = NodeDataLazyClose<T>;

    type Prev = NodeRefSingle<'a, Self, T>;

    type Next = NodeRefSingle<'a, Self, T>;

    type Ends = EndsDoubly<'a, T>;

    type MemoryReclaim = MemoryReclaimOnThreshold<2>;
}

impl<'a, T> ListVariant<'a, T> for Doubly
where
    T: 'a,
{
    #[cfg(test)]
    fn validate(list: &crate::list::List<'a, Self, T>)
    where
        Self::Ends: ListEnds<'a, Self, T>,
    {
        list.validate_list();
    }
}
