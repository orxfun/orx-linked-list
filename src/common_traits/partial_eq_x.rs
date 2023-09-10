use crate::{node::LinkedListNode, LinkedList, LinkedListX};
use orx_imp_vec::prelude::{FixedVec, Growth, PinnedVec, SplitVec};

impl<'a, T, P> PartialEq for LinkedListX<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

// FixedVec
impl<'a, T, P> PartialEq<FixedVec<T>> for LinkedListX<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &FixedVec<T>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}
impl<'a, T, P> PartialEq<LinkedListX<'a, T, P>> for FixedVec<T>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &LinkedListX<'a, T, P>) -> bool {
        other == self
    }
}

// SplitVec
impl<'a, T, P, G> PartialEq<SplitVec<T, G>> for LinkedListX<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
    G: Growth,
{
    fn eq(&self, other: &SplitVec<T, G>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}
impl<'a, T, P, G> PartialEq<LinkedListX<'a, T, P>> for SplitVec<T, G>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
    G: Growth,
{
    fn eq(&self, other: &LinkedListX<'a, T, P>) -> bool {
        other == self
    }
}

// Vec
impl<'a, T, P> PartialEq<LinkedListX<'a, T, P>> for Vec<T>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &LinkedListX<'a, T, P>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}
impl<'a, T, P> PartialEq<Vec<T>> for LinkedListX<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &Vec<T>) -> bool {
        other == self
    }
}

// [T]
impl<'a, T, P> PartialEq<[T]> for LinkedListX<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &[T]) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}
impl<'a, T, P> PartialEq<LinkedListX<'a, T, P>> for [T]
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &LinkedListX<'a, T, P>) -> bool {
        other == self
    }
}

// [T;N]
impl<'a, T, P, const N: usize> PartialEq<[T; N]> for LinkedListX<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &[T; N]) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}
impl<'a, T, P, const N: usize> PartialEq<LinkedListX<'a, T, P>> for [T; N]
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &LinkedListX<'a, T, P>) -> bool {
        other == self
    }
}

// SplitVec
impl<'a, T, P> PartialEq<LinkedList<'a, T, P>> for LinkedListX<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &LinkedList<'a, T, P>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

impl<'a, T, P> PartialEq<LinkedListX<'a, T, P>> for LinkedList<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &LinkedListX<'a, T, P>) -> bool {
        other == self
    }
}
