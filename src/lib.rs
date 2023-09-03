//! An actually useful linked list :) documentation will follow.
//!
//! # Example
//!
//! ```rust
//! use orx_linked_list::prelude::*;
//!
//! let mut list = LinkedList::with_exponential_growth(2, 1.5);
//!
//! // build linked list: x <-> a <-> b <-> c
//! list.push_back('a');
//! list.push_back('b');
//! list.push_front('x');
//! list.push_back('c');
//!
//! assert_eq!(Some('c'), list.pop_back());
//! assert_eq!(Some('b'), list.pop_back());
//! assert_eq!(Some('a'), list.pop_back());
//! assert_eq!(Some('x'), list.pop_back());
//! assert_eq!(None, list.pop_back());
//! assert_eq!(None, list.pop_front());
//! ```

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

mod common_traits;
mod impl_bounded;
mod iterator;
mod linked_list;
mod mem;
mod new;
mod node;
/// Common traits, enums and structs.
pub mod prelude;

pub use crate::linked_list::LinkedList;
pub use crate::mem::{MemoryStatus, MemoryUtilization};
pub use orx_imp_vec::prelude::*;
