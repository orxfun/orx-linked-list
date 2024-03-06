//! # orx-linked-list
//!
//! [![orx-linked-list crate](https://img.shields.io/crates/v/orx-linked-list.svg)](https://crates.io/crates/orx-linked-list)
//! [![orx-linked-list documentation](https://docs.rs/orx-linked-list/badge.svg)](https://docs.rs/orx-linked-list)
//!
//! An efficient and recursive singly and doubly linked list implementation.
//!
//! ## Variants
//!
//! * `type SinglyLinkedList<'a, T> = List<'a, Singly, T>;`
//! * `type DoublyLinkedList<'a, T> = List<'a, Doubly, T>;`
//!
//! ## Time Complexity of Methods
//!
//! In order to indicate the methods available only for the `Doubly` linked list, but not `Singly`, **(*d*)** indicator is used.
//!
//! The following is the list of methods with constant time **O(1)** time complexity.
//!
//! | ***O(1)*** Methods |
//! | -------- |
//! | `front`, `back`: access to front and back of the list  |
//! | `get`: access to to any node with a given index |
//! | `push_front`, `push_back`: push to front or back (*d*) of the list |
//! | `pop_front`, `pop_back`: pop from front and back (*d*) of the list |
//! | `insert_prev_to`, `insert_next_to`: insert a value previous or next to an existing node with a given index (*d*) |
//! | `append_front`, `append_back`: append another list to front or back of the list |
//! | `iter`, `iter_from_back`: create an iterator from the front or back (*d*) of the list; iterating has O(n) time complexity |
//! | `iter_forward_from`, `iter_backward_from`: create a forward or backward (*d*) iterator from any intermediate node with a given index; iterating has O(n) time complexity |
//!
//! | ***O(n)*** Methods |
//! | -------- |
//! | `index_of`: get the index of an element, which can later be used for ***O(1)*** methods |
//! | `contains`, `position_of`: check the existence or position of a value |
//! | `insert_at`: insert an element to an arbitrary position of the list |
//! | `remove_at`: remove an element from an arbitrary position of the list |
//! | `iter`, `iter_from_back`: iterate from the front or back (*d*) of the list |
//! | `iter_forward_from`, `iter_backward_from`: iterate in forward or backward (*d*) direction from any intermediate node with a given index |
//! | `retain`, `retain_collect`: retain keeping elements satisfying a predicate and optionally collect removed elements |
//!
//!
//! ## Examples
//!
//! ### Common Usage
//!
//! `orx_linked_list::List` provides common linked list functionalities, with a special emphasis on maintaining the recursive nature of the data structure which allows for constant time merging of lists.
//!
//! ```rust
//! use orx_linked_list::*;
//!
//! fn eq<'a, I: Iterator<Item = &'a u32> + Clone>(iter: I, slice: &[u32]) -> bool {
//!     iter.clone().count() == slice.len() && iter.zip(slice.iter()).all(|(a, b)| a == b)
//! }
//!
//! let _list: List<Singly, u32> = List::new();
//! let _list = SinglyLinkedList::<u32>::new();
//! let _list: List<Doubly, u32> = List::new();
//! let _list = DoublyLinkedList::<u32>::new();
//!
//! let mut list = DoublyLinkedList::from_iter([3, 4, 5]);
//! assert_eq!(list.front(), Some(&3));
//! assert_eq!(list.back(), Some(&5));
//! assert!(eq(list.iter(), &[3, 4, 5]));
//! assert!(eq(list.iter_from_back(), &[5, 4, 3]));
//!
//! assert_eq!(list.pop_front(), Some(3));
//! assert_eq!(list.pop_back(), Some(5));
//!
//! list.push_back(5);
//! list.push_front(3);
//! assert!(eq(list.iter(), &[3, 4, 5]));
//!
//! let other = DoublyLinkedList::from_iter([6, 7, 8, 9]);
//! list.append_back(other);
//! assert!(eq(list.iter(), &[3, 4, 5, 6, 7, 8, 9]));
//!
//! let other = DoublyLinkedList::from_iter([0, 1, 2]);
//! list.append_front(other);
//! assert!(eq(list.iter(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
//!
//! list.retain(&|x| x < &5);
//! assert!(eq(list.iter(), &[0, 1, 2, 3, 4]));
//!
//! let mut odds = vec![];
//! let mut collect_odds = |x| odds.push(x);
//! list.retain_collect(&|x| x % 2 == 0, &mut collect_odds);
//!
//! assert!(eq(list.iter(), &[0, 2, 4]));
//! assert!(eq(odds.iter(), &[1, 3]));
//! ```
//!
//! ### `NodeIndex` Usage
//!
//! `NodeIndex` allows indexing into the collection in constant time with safety guarantees. The indices returned by growth methods, such as `push_back` or `append_next_to`, can be stored externally or an index for a value can be obtained in linear time with `index_of` method.
//!
//! ```rust
//! use orx_linked_list::*;
//! use orx_selfref_col::NodeIndexError;
//!
//! fn eq<'a, I: Iterator<Item = &'a char> + Clone>(iter: I, slice: &[char]) -> bool {
//!     iter.clone().count() == slice.len() && iter.zip(slice.iter()).all(|(a, b)| a == b)
//! }
//!
//! let mut list = DoublyLinkedList::from_iter(['a', 'b', 'c', 'd']);
//!
//! let x = list.index_of(&'x');
//! assert!(x.is_none());
//!
//! let maybe_b = list.index_of(&'b'); // O(n)
//! assert!(maybe_b.is_some());
//!
//! let b = maybe_b.unwrap();
//!
//! let data_b = list.get(b); // O(1)
//! assert_eq!(data_b, Some(&'b'));
//!
//! // O(1) to create the iterators from the index
//! assert!(eq(list.iter_forward_from(b).unwrap(), &['b', 'c', 'd']));
//! assert!(eq(list.iter_backward_from(b).unwrap(), &['b', 'a']));
//!
//! list.insert_prev_to(b, 'X').unwrap(); // O(1)
//! list.insert_next_to(b, 'Y').unwrap(); // O(1)
//! assert!(eq(list.iter(), &['a', 'X', 'b', 'Y', 'c', 'd']));
//!
//! let removed = list.remove(b); // O(1)
//! assert_eq!(removed, Ok('b'));
//! assert!(eq(list.iter(), &['a', 'X', 'Y', 'c', 'd']));
//!
//! // not possible to wrongly use the index
//! assert_eq!(list.get(b), None);
//! assert_eq!(
//!     list.get_or_error(b).err(),
//!     Some(NodeIndexError::RemovedNode)
//! );
//! ```
//!
//! ## Internal Features
//!
//! `orx_linked_list::List` makes use of the safety guarantees and efficiency features of [SelfRefCol](https://crates.io/crates/orx-selfref-col).
//! * `SelfRefCol` constructs its safety guarantees around the fact that all references will be among elements of the same collection. By preventing bringing in external references or leaking out references, it is safe to build the self referential collection with **regular `&` references**.
//! * With careful encapsulation, `SelfRefCol` prevents passing in external references to the list and leaking within list node references to outside. Once this is established, it provides methods to easily mutate inter list node references. These features allowed a very convenient implementation of the linked list in this crate with almost no use of the `unsafe` keyword, no read or writes through pointers and no access by indices. Compared to the `std::collections::LinkedList` implementation, it can be observed that `orx_linked_list::List` is a much **higher level implementation**.
//! * Furthermore, `orx_linked_list::List` is **significantly faster** than the standard linked list. One of the main reasons for this is the feature of `SelfRefCol` keeping all close to each other rather than at arbitrary locations in memory which leads to a better cache locality.
//!
//! ## Benchmarks
//!
//! ### Mutation Ends
//!
//! *You may see the benchmark at [benches/mutation_ends.rs](https://github.com/orxfun/orx-linked-list/blob/main/benches/mutation_ends.rs).*
//!
//! This benchmark compares time performance of calls to `push_front`, `push_back`, `pop_front` and `pop_back` methods.
//!
//! <img src="https://raw.githubusercontent.com/orxfun/orx-linked-list/main/docs/img/bench_mutation_ends.PNG" alt="https://raw.githubusercontent.com/orxfun/orx-linked-list/main/docs/img/bench_mutation_ends.PNG" />
//!
//! ### Iteration
//!
//! *You may see the benchmark at [benches/iter.rs](https://github.com/orxfun/orx-linked-list/blob/main/benches/iter.rs).*
//!
//! This benchmark compares time performance of iteration through the `iter` method.
//!
//! <img src="https://raw.githubusercontent.com/orxfun/orx-linked-list/main/docs/img/iter.PNG" alt="https://raw.githubusercontent.com/orxfun/orx-linked-list/main/docs/img/iter.PNG" />
//!
//! ## License
//!
//! This library is licensed under MIT license. See LICENSE for details.

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
mod iterators;
mod list;
mod option_utils;
mod variants;

pub use list::{DoublyLinkedList, List, SinglyLinkedList};
pub use variants::{doubly::Doubly, singly::Singly};
