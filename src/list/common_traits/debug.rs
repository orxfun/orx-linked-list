use crate::{variant::Doubly, DoublyIterable, List, Singly, SinglyIterable};
use core::fmt::Debug;
use orx_selfref_col::MemoryPolicy;

impl<T: Debug, M> Debug for List<Singly<T>, M>
where
    M: MemoryPolicy<Singly<T>>,
    List<Singly<T>, M>: Default,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[")?;

        let mut iter = self.iter();
        if let Some(first) = iter.next() {
            write!(f, "{:?}", first)?;
            for x in iter {
                write!(f, " -> {:?}", x)?;
            }
        }

        write!(f, "]")
    }
}

impl<T: Debug, M> Debug for List<Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
    List<Doubly<T>, M>: Default,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[")?;

        let mut iter = self.iter();
        if let Some(first) = iter.next() {
            write!(f, "{:?}", first)?;
            for x in iter {
                write!(f, " <-> {:?}", x)?;
            }
        }

        write!(f, "]")
    }
}
