#![doc = include_str!("../README.md")]
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
#![no_std]

#[cfg(any(test, feature = "validation"))]
mod tests;

#[cfg(any(test, feature = "validation"))]
extern crate std;

extern crate alloc;

/// Module containing iterators from the list.
pub mod iter;
mod list;
mod memory;
/// Module providing access to the pointers of the linked list nodes.
pub mod pointers;
mod type_aliases;
mod variant;

pub use list::List;
pub use list::ends_traits::*;
pub use list::iter_traits::*;
pub use list::slice::{ListSlice, ListSliceMut};
pub use orx_selfref_col::{MemoryPolicy, NodeIdx, NodeIdxError};
pub use type_aliases::{
    DoublyIdx, DoublyList, DoublyListLazy, DoublyListSlice, DoublyListSliceLazy,
    DoublyListSliceMut, DoublyListSliceMutLazy, DoublyListThreshold, SinglyIdx, SinglyList,
    SinglyListLazy, SinglyListSlice, SinglyListSliceLazy, SinglyListSliceMut,
    SinglyListSliceMutLazy, SinglyListThreshold,
};
pub use variant::{Doubly, Singly};

#[cfg(feature = "orx-parallel")]
pub use orx_parallel::*;
