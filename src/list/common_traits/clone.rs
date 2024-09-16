use crate::{variant::Doubly, DoublyIterable, List, Singly, SinglyIterable};
use orx_selfref_col::MemoryPolicy;

impl<T: Clone, M> Clone for List<Singly<T>, M>
where
    M: MemoryPolicy<Singly<T>>,
    List<Singly<T>, M>: Default,
{
    fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
}

impl<T: Clone, M> Clone for List<Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
    List<Doubly<T>, M>: Default,
{
    fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
}
