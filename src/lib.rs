//! # orx-linked-list
//!
//! Implements a doubly-linked list.
//!
//! As opposed to the note of `std::collections::LinkedList`, `orx_linked_list::LinkedList` provides the main theoretical benefits of a linked list; i.e.,
//!  
//! * efficient insertion and removal of elements,
//!  
//! while aiming to avoid the practical drawbacks related with allocations and CPU cache due to the following:
//!
//! * `LinkedList` uses an [`ImpVec`](https://crates.io/crates/orx-imp-vec) as the underlying data structure which allows for defining inter-element relationships by thin references; `next` and `prev` relationships are defined by thin `&` references:
//!     * without any additional heap allocations since smart pointers such as `Box` or `Rc` per element are not necessary;
//!     * or without the need to use plain index numbers to mimic references.
//! * All elements are stored in the underlying [`PinnedVec`](https://crates.io/crates/orx-pinned-vec) of the `ImpVec`; which might either be a [`FixedVec`](https://crates.io/crates/orx-fixed-vec) or [`SplitVec`](https://crates.io/crates/orx-split-vec). In either case, the elements will be stored close to each other in a vec-like structure. Although the order of elements in this vector will not be in the correct order as expected in a linked list, they will be pointing to other elements of the same vector. Therefore, unlike classical implementations by arbitrary heap allocations, the `LinkedList` implementation provides better cache locality.
//!
//! ## Usage
//!
//! Basic usage of the linked list is demonstrated below.
//!
//! ```rust
//! use orx_linked_list::prelude::*;
//!
//! mut list = LinkedList::with_doubling_growth(4);
//!
//! list.push_back('y');
//! list.push_front('x');
//! list.push_back('z');
//! assert_eq!(vec!['x', 'y', 'z'], list.collect_vec());
//!
//! assert_eq!(list.pop_back(), Some('z'));
//! assert_eq!(list.pop_front(), Some('x'));
//! assert_eq!(vec!['y'], list.collect_vec());
//!
//! list.push_front('x');
//! list.push_back('z');
//! assert_eq!(vec!['x', 'y', 'z'], list.collect_vec());
//!
//! list.insert_at(1, '?');
//! assert_eq!(vec!['x', '?', 'y', 'z'], list.collect_vec());
//!
//! assert_eq!(Some(&'?'), list.get_at(1));
//! *list.get_mut_at(1).unwrap() = '!';
//!
//! assert_eq!('!', list.remove_at(1));
//! assert_eq!(vec!['x', 'y', 'z'], list.collect_vec());
//! ```
//!
//! ## Memory
//!
//! `LinkedList` provides two ways to configure the memory strategy:
//!
//! * the first configuration is inherited from the `PinnedVec` variants and defines how the underlying storage will be kept;
//! * the second is related with the tradeoff between memory utilization and laziness favouring faster operations.
//!
//! ### Underlying Storage Variants
//!
//! The complete signature of a `LinkedList` holding elements of type `T` is as follows:
//!
//! ```rust ignore
//! LinkedList<'a, T, P> where P: PinnedVec<LinkedListNode<'a, T>>
//! ```
//!
//! The choice of the underlying `PinnedVec` defines the dynamic allocations. See [`FixedVec`](https://crates.io/crates/orx-fixed-vec) or [`SplitVec`](https://crates.io/crates/orx-split-vec) for possible strategies.
//!
//! The following type aliases are defined for convenience to simplify the type signatures:
//!
//! ```rust ignore
//! pub type LinkedListFixed<'a, T>
//!     = LinkedList<'a, T, FixedVec<LinkedListNode<'a, T>>>;
//!
//! pub type LinkedListLinear<'a, T>
//!     = LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, Linear>>;
//!
//! pub type LinkedListDoubling<'a, T>
//!     = LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, Doubling>>;
//!
//! pub type LinkedListExponential<'a, T>
//!     = LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, Exponential>>;
//! ```
//!
//! ### Time Complexity vs Memory Utilization
//!
//! `LinkedList` holds all elements close to each other in a `PinnedVec` aiming for better cache locality while using thin references rather than wide pointers and to reduce heap allocations. In order to achieve *O(1)* time complexity while avoiding smart pointers, remove and pop operations are designed to be semi-lazy.
//!
//! In the lazy case; i.e., when the strategy is set to `MemoryReclaimStrategy::Lazy`, every `pop_back`, `pop_front` or `remove_at` operation leaves a gap in the underlying vector. Status of utilization of the underlying vector can be queried using the `memory_status` method and the gaps can completely be reclaimed by manually calling the `memory_reclaim` method which has a time complexity of *O(n)* where *n* is the length of the underlying vector.
//!
//! Being able to be lazy and to reclaim the gaps, it is possible to define and use different automated strategies which would fit better in different situations:
//!
//! * `Lazy`: `memory_reclaim` is never called automatically:
//!     * leads to the cheapest possible `pop_back`, `pop_front` or `remove_at` operations,
//!     * however, the utilization of the vector can be low especially when a large number of elements enter and exit the linked list.
//!     * might be a better fit where keeping the time complexity of these operations at *O(1)* is important; or when utilization is not expected to drop too low.
//! * `Eager`: every `pop_back`, `pop_front` or `remove_at` method call is automatically followed by a `memory_reclaim` call:
//!     * this strategy continuously keeps the vector without gaps at 100% utilization;
//!     * however, abovementioned operations require *O(n)* time complexity;
//!     * might be a better fit where memory is scarce and more important than the increased time-complexity of these methods.
//! * `WithThreshold(threshold)`: `pop_back`, `pop_front` or `remove_at` method call is followed by an automatic `memory_reclaim` call only if the memory utilization drops below a pre-determined `threshold`:
//!     * it is a generalization of `Lazy` and `Eager` allowing to select the required threshold level between memory utilization and amortized time complexity of these methods. Note that setting the least memory utilization to a value lower than 1.0 would still least to a constant amortized time complexity.
//!

#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]

mod collect;
mod common_traits;
mod fixed_vec;
mod impl_bounded;
mod iterator;
mod linked_list;
mod mem;
mod new;
mod node;
/// Common traits, enums and structs.
pub mod prelude;
mod variants;

pub use crate::linked_list::LinkedList;
pub use crate::mem::{MemoryStatus, MemoryUtilization};
pub use crate::variants::{
    LinkedListDoubling, LinkedListExponential, LinkedListFixed, LinkedListLinear,
};
