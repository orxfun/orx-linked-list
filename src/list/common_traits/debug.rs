use crate::{DoublyIterable, List, Singly, SinglyIterable, variant::Doubly};
use core::fmt::Debug;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node};

impl<T: Debug, M, P> Debug for List<Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
    List<Singly<T>, M, P>: Default,
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

impl<T: Debug, M, P> Debug for List<Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
    List<Doubly<T>, M, P>: Default,
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
