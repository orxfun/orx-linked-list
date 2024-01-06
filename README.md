# orx-linked-list

An efficient doubly linked list using regular `&` references with a focus to avoid smart pointers and improve cache locality.

## A. Motivation

Self referential, often recursive, collections contain an important set of useful data structures including linked lists. However, building these structures with references `&` is not possible in safe rust.

Alternatively, these collections can be built using reference counted smart pointers such as `std::rc::Rc` and independent heap allocations. However, independent heap allocations is a drawback as the elements do not live close to each other leading to poor cache locality. Further, reference counted pointers have a runtime overhead. This crate makes use of [`orx_imp_vec::ImpVec`](https://crates.io/crates/orx-imp-vec) as the underlying storage. ImpVec specializes in enabling self referential collections by relying on the pinned memory location guarantee provided by the [`orx_pinned_vec::PinnedVec`](https://crates.io/crates/orx-pinned-vec).

### Standard LinkedList

Standard `std::collections::LinkedList` implementation avoids reference counted pointers and uses `NonNull` instead, most likely to avoid this overhead. However, this leads to a risky and difficult implementation that feels more low level than it should. You may see the implementation [here](https://doc.rust-lang.org/src/alloc/collections/linked_list.rs.html). The `unsafe` keyword is used more than 60 times in this file. These are usually related to reading from and writing to memory through raw pointers.

***Motivation:*** We do not need to count references provided that all elements and inter-element references belong to the same owner or container. This is because all elements will be dropped at the same time together with their inter-element references when the container `ImpVec` is dropped.

***Motivation:*** We should be able to define these structures without directly accessing memory through raw pointers. This is unnecessarily powerful and risky. Instead, unsafe code must be limited to methods which are specialized for and only allow defining required connections of self referential collections.

### orx_linked_list::LinkedList

Linked list implementation in this crate uses an `ImpVec` as the underlying storage and makes use of its specialized methods. This brings the following advantages:

* Allows for a higher level implementation without any use of raw pointers.
* Avoids smart pointers.
* Avoids almost completely accessing through integer indices.
* All nodes belong to the same `ImpVec` living close to each other. This allows for better cache locality.
* Full fetched doubly-linked-list implementation uses the `unsafe` keyword seven times, which are repeated uses of three methods:
  * `ImpVec::push_get_ref`
  * `ImpVec::move_get_ref`
  * `ImpVec::unsafe_truncate` (*a deref method from [`PinnedVec`](https://crates.io/crates/orx-pinned-vec)*)

Furthermore, this implementation is more performant than the standard library implementation, a likely indicator of better cache locality. You may below the benchmark results for a series of random push/pop mutations after pushing "number of elements" elements to the list.

<img src="https://raw.githubusercontent.com/orxfun/orx-linked-list/main/docs/img/bench_mutation_ends.PNG" alt="https://raw.githubusercontent.com/orxfun/orx-linked-list/main/docs/img/bench_mutation_ends.PNG" />

However, note that the benchmark compares only the linked list implementations. `std::collections::VecDeque` is significantly more efficient than both linked lists for most operations. Therefore, it is preferrable unless the flexibility of linked list's recursive nature is not required (see `split` methods in the next section).

## B. Features

`orx_linked_list::LinkedList` implementation provides standard linked list opeartions such as constant time insertions and removals. Further, it reflects the **recursive** nature of the data structure through so called `LinkedListSlice`s. The caller can move to the desired element of the linked list and get the rest of the list as a linked list slice; which is nothing but an immutable linked list. Furthermore, slices can simply be `collect`ed as an owned linked list.

```rust
use orx_linked_list::*;

// BASIC
let mut list = LinkedList::new();
list.extend(vec!['a', 'b', 'c']);

assert_eq!(list.len(), 3);
assert!(list.contains(&'b'));
assert_eq!(list.index_of(&'c'), Some(2));
assert_eq!(list.from_back_index_of(&'c'), Some(0));

list.push_back('d');
assert_eq!(Some(&'d'), list.back());

*list.get_at_mut(0).unwrap() = 'x';

list.push_front('e');
*list.front_mut().unwrap() = 'f';

_ = list.remove_at(1);
_ = list.pop_back();
list.insert_at(0, 'x');
list.clear();
list.push_front('y');
list.pop_front();

// ITER
let list: LinkedList<_> = ['a', 'b', 'c', 'd', 'e'].into_iter().collect();

let forward: Vec<_> = list.iter().copied().collect();
assert_eq!(forward, &['a', 'b', 'c', 'd', 'e']);

let backward: Vec<_> = list.iter_from_back().copied().collect();
assert_eq!(backward, &['e', 'd', 'c', 'b', 'a']);

// SPLITS
let (left, right) = list.split(2).unwrap();
assert_eq!(left, &['a', 'b']);
assert_eq!(right, &['c', 'd', 'e']);
// left & right are also nothing but immutable linked lists
assert_eq!(right.front(), Some(&'c'));
assert_eq!(left.back(), Some(&'b'));

let (front, after) = list.split_front().unwrap();
assert_eq!(front, &'a');
assert_eq!(after, &['b', 'c', 'd', 'e']);

let (before, back) = list.split_back().unwrap();
assert_eq!(before, &['a', 'b', 'c', 'd']);
assert_eq!(back, &'e');

let (left, right) = list.split_before(&'d').unwrap();
assert_eq!(left, &['a', 'b', 'c']);
assert_eq!(right, &['d', 'e']);

let (left, right) = list.split_after(&'d').unwrap();
assert_eq!(left, &['a', 'b', 'c', 'd']);
assert_eq!(right, &['e']);

// RECURSIVE SPLITS
let (left1, left2) = left.split(1).unwrap();
assert_eq!(left1, &['a']);
assert_eq!(left2, &['b', 'c', 'd']);

// SPLIT TO OWNED
let mut left_list = left.collect();

assert_eq!(left_list, &['a', 'b', 'c', 'd']);
_ = left_list.pop_front();
_ = left_list.pop_back();
assert_eq!(left_list, &['b', 'c']);
```

## License

This library is licensed under MIT license. See LICENSE for details.
