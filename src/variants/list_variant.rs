use orx_selfref_col::{NodeDataLazyClose, NodeRefSingle, NodeRefsArray, Variant};

pub trait ListVariant<'a, T>:
    Variant<
    'a,
    T,
    Storage = NodeDataLazyClose<T>,
    Next = NodeRefSingle<'a, Self, T>,
    Ends = NodeRefsArray<'a, 2, Self, T>,
>
where
    Self: 'a,
    T: 'a,
{
    #[cfg(test)]
    fn validate(list: &crate::list::List<'a, Self, T>)
    where
        Self::Ends: crate::variants::ends::ListEnds<'a, Self, T>;
}
