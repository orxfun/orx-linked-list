//! An actually useful linked list :) documentation will follow.

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
mod new;
mod node;
pub mod prelude;
mod utilization;
