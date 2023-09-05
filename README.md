# orx-linked-list

Implements a doubly-linked list.

As opposed to the note of `std::collections::LinkedList`, `orx_linked_list::LinkedList` provides the main theoretical benefits of a linked list; i.e.,
 
* efficient insertion and removal of elements,
 
while aiming to avoid the practical drawbacks related with allocations and CPU cache due to the following:

* `LinkedList` uses an [`ImpVec`](https://crates.io/crates/orx-imp-vec) as the underlying data structure which allows for defining inter-element relationships by thin references;
    * `next` and `prev` relationships are defined by thin `&` references without any additional heap allocations since smart pointers such as `Box` or `Rc` per element are not necessary.
* All elements are stored in the underlying [`PinnedVec`](https://crates.io/crates/orx-pinned-vec) of the `ImpVec`; which might either be a [`FixedVec`](https://crates.io/crates/orx-fixed-vec) or [`SplitVec`](https://crates.io/crates/orx-split-vec). In either case, the elements will be stored close to each other in a vec-like structure. Although the order of elements in this vector will not be in the correct order as expected in a linked list, they will be pointing to other elements of the same vector. Therefore, unlike classical implementations by arbitrary heap allocations, the `LinkedList` implementation provides better cache locality.

# Example

```rust
use orx_linked_list::prelude::*;

let mut list = LinkedList::with_exponential_growth(2, 1.5, MemoryUtilization::default());

// build linked list: x <-> a <-> b <-> c
list.push_back('a');
list.push_back('b');
list.push_front('x');
list.push_back('c');

assert_eq!(Some('c'), list.pop_back());
assert_eq!(Some('b'), list.pop_back());
assert_eq!(Some('a'), list.pop_back());
assert_eq!(Some('x'), list.pop_back());
assert_eq!(None, list.pop_back());
assert_eq!(None, list.pop_front());
```