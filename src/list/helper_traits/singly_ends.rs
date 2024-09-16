use super::{HasCol, HasColMut};
use crate::Singly;
use orx_selfref_col::{MemoryPolicy, Variant};

/// Lists and views with owned ends.
pub trait HasSinglyEnds<T, M>: HasCol<Singly<T>, M>
where
    M: MemoryPolicy<Singly<T>>,
{
    /// Returns a reference to the ends of the linked list.
    fn ends(&self) -> &<Singly<T> as Variant>::Ends;
}

/// Lists and views with owned mutable ends.
pub trait HasSinglyEndsMut<T, M>: HasColMut<Singly<T>, M> + HasSinglyEnds<T, M>
where
    M: MemoryPolicy<Singly<T>>,
{
    /// Returns a mutable reference to the ends of the linked list.
    fn ends_mut(&mut self) -> &mut <Singly<T> as Variant>::Ends;
}
