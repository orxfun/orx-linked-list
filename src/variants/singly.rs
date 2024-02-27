use super::{ends::ListEnds, list_variant::ListVariant};
use orx_selfref_col::{
    MemoryReclaimOnThreshold, Node, NodeDataLazyClose, NodeRefNone, NodeRefSingle, NodeRefs,
    NodeRefsArray, Variant,
};

pub type EndsSingly<'a, T> = NodeRefsArray<'a, 2, Singly, T>;

impl<'a, T> ListEnds<'a, Singly, T> for EndsSingly<'a, T> {
    fn front(&self) -> Option<&'a Node<'a, Singly, T>> {
        self.get()[0]
    }

    fn back(&self) -> Option<&'a Node<'a, Singly, T>> {
        self.get()[1]
    }
}

/// A singly linked list variant such that:
/// * Each node contains two data elements: the value of the element and a reference to the next node.
/// * The list keeps track of its `front`.
/// * It is possible to iterate from the `front` to the back of the list.
pub struct Singly;

impl<'a, T> Variant<'a, T> for Singly
where
    T: 'a,
{
    type Storage = NodeDataLazyClose<T>;

    type Prev = NodeRefNone;

    type Next = NodeRefSingle<'a, Self, T>;

    type Ends = EndsSingly<'a, T>;

    type MemoryReclaim = MemoryReclaimOnThreshold<2>;
}

impl<'a, T> ListVariant<'a, T> for Singly
where
    T: 'a,
{
    #[cfg(test)]
    fn validate(list: &crate::list::List<'a, Self, T>)
    where
        Self::Ends: crate::variants::ends::ListEnds<'a, Self, T>,
    {
        list.validate_list();
    }
}
