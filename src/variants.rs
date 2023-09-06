use crate::{node::LinkedListNode, LinkedList};
use orx_imp_vec::prelude::*;

/// Type alias for `LinkedList<'a, T, FixedVec<LinkedListNode<'a, T>>>`
/// used to simplify the type signature.
///
/// It is a `LinkedList` whose underlying `ImpVec` uses a `FixedVec` as the underlying pinned storage.
pub type LinkedListFixed<'a, T> = LinkedList<'a, T, FixedVec<LinkedListNode<'a, T>>>;
/// Type alias for `LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, Linear>>`
/// used to simplify the type signature.
///
/// It is a `LinkedList` whose underlying `ImpVec` uses a `SplitVec` with `Linear` growth as the underlying pinned storage.
pub type LinkedListLinear<'a, T> = LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, Linear>>;
/// Type alias for `LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, Doubling>>`
/// used to simplify the type signature.
///
/// It is a `LinkedList` whose underlying `ImpVec` uses a `SplitVec` with `Doubling` growth as the underlying pinned storage.
pub type LinkedListDoubling<'a, T> = LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, Doubling>>;
/// Type alias for `LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, Exponential>>`
/// used to simplify the type signature.
///
/// It is a `LinkedList` whose underlying `ImpVec` uses a `SplitVec` with `Exponential` growth as the underlying pinned storage.
pub type LinkedListExponential<'a, T> =
    LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, Exponential>>;
