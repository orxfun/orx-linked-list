use crate::{
    ListSlice, ListSliceMut,
    list::List,
    variant::{Doubly, ListVariant, Singly},
};
use orx_selfref_col::{MemoryReclaimNever, MemoryReclaimOnThreshold, Node, NodeIdx};
use orx_split_vec::{Recursive, SplitVec};

// crate
#[allow(type_alias_bounds)]
pub(crate) type DefaultMemory<V: ListVariant> = MemoryReclaimOnThreshold<2, V, V::Reclaimer>;

pub(crate) type DefaultPinVec<V> = SplitVec<Node<V>, Recursive>;

pub(crate) const FRONT_IDX: usize = 0;
pub(crate) const BACK_IDX: usize = 1;

pub(crate) const OOB: &str = "out-of-bounds";
pub(crate) const IDX_ERR: &str = "invalid index";

// pub

/// A singly linked list with default memory reclaim policy:
/// * nodes hold a reference to the next element, but not to the previous;
/// * memory of removed nodes are automatically reclaimed when utilization falls below 75%.
///
/// Importantly note that methods of the the following traits are also available:
/// * [`SinglyEnds`]
/// * [`SinglyEndsMut`]
/// * [`SinglyIterable`]
/// * [`SinglyIterableMut`]
///
/// [`SinglyEnds`]: crate::SinglyEnds
/// [`SinglyEndsMut`]: crate::SinglyEndsMut
/// [`SinglyIterable`]: crate::SinglyIterable
/// [`SinglyIterableMut`]: crate::SinglyIterableMut
pub type SinglyList<T, P = DefaultPinVec<Singly<T>>> = List<Singly<T>, DefaultMemory<Singly<T>>, P>;

/// A doubly linked list with default memory reclaim policy:
/// * nodes hold a reference to the next element, and a reference to the previous;
/// * memory of removed nodes are automatically reclaimed when utilization falls below 75%.
///
/// Importantly note that methods of the the following traits are also available:
/// * [`DoublyEnds`]
/// * [`DoublyEndsMut`]
/// * [`DoublyIterable`]
/// * [`DoublyIterableMut`]
///
/// [`DoublyEnds`]: crate::DoublyEnds
/// [`DoublyEndsMut`]: crate::DoublyEndsMut
/// [`DoublyIterable`]: crate::DoublyIterable
/// [`DoublyIterableMut`]: crate::DoublyIterableMut
pub type DoublyList<T, P = DefaultPinVec<Doubly<T>>> = List<Doubly<T>, DefaultMemory<Doubly<T>>, P>;

/// A singly linked list with lazy memory reclaim policy:
/// * nodes hold a reference to the next element, but not to the previous;
/// * memory of removed nodes are never reclaimed implicitly, the caller can explicitly reclaim by calling `reclaim_closed_nodes`,
///   * this guarantees that indices will never be invalidated implicitly.
///
/// Importantly note that methods of the the following traits are also available:
/// * [`SinglyEnds`]
/// * [`SinglyEndsMut`]
/// * [`SinglyIterable`]
/// * [`SinglyIterableMut`]
///
/// [`SinglyEnds`]: crate::SinglyEnds
/// [`SinglyEndsMut`]: crate::SinglyEndsMut
/// [`SinglyIterable`]: crate::SinglyIterable
/// [`SinglyIterableMut`]: crate::SinglyIterableMut
pub type SinglyListLazy<T, P = DefaultPinVec<Singly<T>>> = List<Singly<T>, MemoryReclaimNever, P>;

/// A doubly linked list with lazy memory reclaim policy:
/// * nodes hold a reference to the next element, and a reference to the previous;
/// * memory of removed nodes are never reclaimed implicitly, the caller can explicitly reclaim by calling `reclaim_closed_nodes`,
///   * this guarantees that indices will never be invalidated implicitly.
///
/// Importantly note that methods of the the following traits are also available:
/// * [`DoublyEnds`]
/// * [`DoublyEndsMut`]
/// * [`DoublyIterable`]
/// * [`DoublyIterableMut`]
///
/// [`DoublyEnds`]: crate::DoublyEnds
/// [`DoublyEndsMut`]: crate::DoublyEndsMut
/// [`DoublyIterable`]: crate::DoublyIterable
/// [`DoublyIterableMut`]: crate::DoublyIterableMut
pub type DoublyListLazy<T, P = DefaultPinVec<Doubly<T>>> = List<Doubly<T>, MemoryReclaimNever, P>;

/// A singly linked list with custom memory reclaim on threshold policy:
/// * nodes hold a reference to the next element, but not to the previous;
/// * memory of removed nodes are automatically reclaimed when the ratio of closed nodes to all nodes exceeds one over `2^D`:
///   * when `D = 0`: memory will be reclaimed when utilization is below 0.00% (equivalent to never).
///   * when `D = 1`: memory will be reclaimed when utilization is below 50.00%.
///   * when `D = 2`: memory will be reclaimed when utilization is below 75.00%.
///   * when `D = 3`: memory will be reclaimed when utilization is below 87.50%.
///   * when `D = 4`: memory will be reclaimed when utilization is below 93.75%.
///   * ...
///
/// Importantly note that methods of the the following traits are also available:
/// * [`SinglyEnds`]
/// * [`SinglyEndsMut`]
/// * [`SinglyIterable`]
/// * [`SinglyIterableMut`]
///
/// [`SinglyEnds`]: crate::SinglyEnds
/// [`SinglyEndsMut`]: crate::SinglyEndsMut
/// [`SinglyIterable`]: crate::SinglyIterable
/// [`SinglyIterableMut`]: crate::SinglyIterableMut
pub type SinglyListThreshold<const D: usize, T, P = DefaultPinVec<Singly<T>>> = List<
    Singly<T>,
    MemoryReclaimOnThreshold<D, Singly<T>, <Singly<T> as ListVariant>::Reclaimer>,
    P,
>;

/// A doubly linked list with lazy memory reclaim policy:
/// * nodes hold a reference to the next element, and a reference to the previous;
/// * memory of removed nodes are automatically reclaimed when the ratio of closed nodes to all nodes exceeds one over `2^D`:
///   * when `D = 0`: memory will be reclaimed when utilization is below 0.00% (equivalent to never).
///   * when `D = 1`: memory will be reclaimed when utilization is below 50.00%.
///   * when `D = 2`: memory will be reclaimed when utilization is below 75.00%.
///   * when `D = 3`: memory will be reclaimed when utilization is below 87.50%.
///   * when `D = 4`: memory will be reclaimed when utilization is below 93.75%.
///   * ...
///
/// Importantly note that methods of the the following traits are also available:
/// * [`DoublyEnds`]
/// * [`DoublyEndsMut`]
/// * [`DoublyIterable`]
/// * [`DoublyIterableMut`]
///
/// [`DoublyEnds`]: crate::DoublyEnds
/// [`DoublyEndsMut`]: crate::DoublyEndsMut
/// [`DoublyIterable`]: crate::DoublyIterable
/// [`DoublyIterableMut`]: crate::DoublyIterableMut
pub type DoublyListThreshold<const D: usize, T, P = DefaultPinVec<Doubly<T>>> = List<
    Doubly<T>,
    MemoryReclaimOnThreshold<D, Doubly<T>, <Doubly<T> as ListVariant>::Reclaimer>,
    P,
>;

/// An index to an element on a singly linked list which allows safe and constant time access.
pub type SinglyIdx<T> = NodeIdx<Singly<T>>;

/// An index to an element on a doubly linked list which allows safe and constant time access.
pub type DoublyIdx<T> = NodeIdx<Doubly<T>>;

/// A slice of a singly linked list with default memory reclaim policy:
/// * nodes hold a reference to the next element, but not to the previous;
/// * memory of removed nodes are automatically reclaimed when utilization falls below 75%.
///
/// Importantly note that methods of the the following traits are also available:
/// * [`SinglyEnds`]
/// * [`SinglyIterable`]
///
/// [`SinglyEnds`]: crate::SinglyEnds
/// [`SinglyIterable`]: crate::SinglyIterable
pub type SinglyListSlice<'a, T> = ListSlice<'a, Singly<T>, DefaultMemory<Singly<T>>>;

/// A mutable slice of a singly linked list with default memory reclaim policy:
/// * nodes hold a reference to the next element, but not to the previous;
/// * memory of removed nodes are automatically reclaimed when utilization falls below 75%.
///
/// Importantly note that methods of the the following traits are also available:
/// * [`SinglyEnds`]
/// * [`SinglyEndsMut`]
/// * [`SinglyIterable`]
/// * [`SinglyIterableMut`]
///
/// [`SinglyEnds`]: crate::SinglyEnds
/// [`SinglyEndsMut`]: crate::SinglyEndsMut
/// [`SinglyIterable`]: crate::SinglyIterable
/// [`SinglyIterableMut`]: crate::SinglyIterableMut
pub type SinglyListSliceMut<'a, T> = ListSliceMut<'a, Singly<T>, DefaultMemory<Singly<T>>>;

/// A slice of a doubly linked list with default memory reclaim policy:
/// * nodes hold a reference to the next element, and a reference to the previous;
/// * memory of removed nodes are automatically reclaimed when utilization falls below 75%.
///
/// Importantly note that methods of the the following traits are also available:
/// * [`DoublyEnds`]
/// * [`DoublyIterable`]
///
/// [`DoublyEnds`]: crate::DoublyEnds
/// [`DoublyIterable`]: crate::DoublyIterable
pub type DoublyListSlice<'a, T> = ListSlice<'a, Doubly<T>, DefaultMemory<Doubly<T>>>;

/// A mutable slice of a doubly linked list with default memory reclaim policy:
/// * nodes hold a reference to the next element, and a reference to the previous;
/// * memory of removed nodes are automatically reclaimed when utilization falls below 75%.
///
/// Importantly note that methods of the the following traits are also available:
/// * [`DoublyEnds`]
/// * [`DoublyEndsMut`]
/// * [`DoublyIterable`]
/// * [`DoublyIterableMut`]
///
/// [`DoublyEnds`]: crate::DoublyEnds
/// [`DoublyEndsMut`]: crate::DoublyEndsMut
/// [`DoublyIterable`]: crate::DoublyIterable
/// [`DoublyIterableMut`]: crate::DoublyIterableMut
pub type DoublyListSliceMut<'a, T> = ListSliceMut<'a, Doubly<T>, DefaultMemory<Doubly<T>>>;

/// A slice of a singly linked list with lazy memory reclaim policy:
/// * nodes hold a reference to the next element, but not to the previous;
/// * memory of removed nodes are never reclaimed implicitly, the caller can explicitly reclaim by calling `reclaim_closed_nodes`,
///   * this guarantees that indices will never be invalidated implicitly.
///
/// Importantly note that methods of the the following traits are also available:
/// * [`SinglyEnds`]
/// * [`SinglyIterable`]
///
/// [`SinglyEnds`]: crate::SinglyEnds
/// [`SinglyIterable`]: crate::SinglyIterable
pub type SinglyListSliceLazy<'a, T> = ListSlice<'a, Singly<T>, MemoryReclaimNever>;

/// A mut slice of a singly linked list with lazy memory reclaim policy:
/// * nodes hold a reference to the next element, but not to the previous;
/// * memory of removed nodes are never reclaimed implicitly, the caller can explicitly reclaim by calling `reclaim_closed_nodes`,
///   * this guarantees that indices will never be invalidated implicitly.
///
/// Importantly note that methods of the the following traits are also available:
/// * [`SinglyEnds`]
/// * [`SinglyEndsMut`]
/// * [`SinglyIterable`]
/// * [`SinglyIterableMut`]
///
/// [`SinglyEnds`]: crate::SinglyEnds
/// [`SinglyEndsMut`]: crate::SinglyEndsMut
/// [`SinglyIterable`]: crate::SinglyIterable
/// [`SinglyIterableMut`]: crate::SinglyIterableMut
pub type SinglyListSliceMutLazy<'a, T> = ListSlice<'a, Singly<T>, MemoryReclaimNever>;

/// A slice of a doubly linked list with lazy memory reclaim policy:
/// * nodes hold a reference to the next element, and a reference to the previous;
/// * memory of removed nodes are never reclaimed implicitly, the caller can explicitly reclaim by calling `reclaim_closed_nodes`,
///   * this guarantees that indices will never be invalidated implicitly.
///
/// Importantly note that methods of the the following traits are also available:
/// * [`DoublyEnds`]
/// * [`DoublyIterable`]
///
/// [`DoublyEnds`]: crate::DoublyEnds
/// [`DoublyIterable`]: crate::DoublyIterable
pub type DoublyListSliceLazy<'a, T> = ListSlice<'a, Doubly<T>, MemoryReclaimNever>;

/// A mutable slice of a doubly linked list with lazy memory reclaim policy:
/// * nodes hold a reference to the next element, and a reference to the previous;
/// * memory of removed nodes are never reclaimed implicitly, the caller can explicitly reclaim by calling `reclaim_closed_nodes`,
///   * this guarantees that indices will never be invalidated implicitly.
///
/// Importantly note that methods of the the following traits are also available:
/// * [`DoublyEnds`]
/// * [`DoublyEndsMut`]
/// * [`DoublyIterable`]
/// * [`DoublyIterableMut`]
///
/// [`DoublyEnds`]: crate::DoublyEnds
/// [`DoublyEndsMut`]: crate::DoublyEndsMut
/// [`DoublyIterable`]: crate::DoublyIterable
/// [`DoublyIterableMut`]: crate::DoublyIterableMut
///
/// Importantly note that methods of the the following traits are also available:
/// * [`SinglyEnds`]
/// * [`SinglyEndsMut`]
/// * [`SinglyIterable`]
/// * [`SinglyIterableMut`]
///
/// [`SinglyEnds`]: crate::SinglyEnds
/// [`SinglyEndsMut`]: crate::SinglyEndsMut
/// [`SinglyIterable`]: crate::SinglyIterable
/// [`SinglyIterableMut`]: crate::SinglyIterableMut
pub type DoublyListSliceMutLazy<'a, T> = ListSliceMut<'a, Doubly<T>, MemoryReclaimNever>;
