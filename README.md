# orx-linked-list
An actually useful linked list :) documentation will follow.

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