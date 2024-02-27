use super::list_variant::ListVariant;
use orx_selfref_col::{Node, NodeRefsArray};

pub trait ListEnds<'a, V, T>
where
    T: 'a,
    V: ListVariant<'a, T, Ends = NodeRefsArray<'a, 2, V, T>>,
{
    fn front(&self) -> Option<&'a Node<'a, V, T>>;

    fn back(&self) -> Option<&'a Node<'a, V, T>>;
}
