# orx-linked-list

An efficient and recursive singly and doubly linked list implementation.

## Variants and Time Complexity of Methods

* `type SinglyLinkedList<'a, T> = List<'a, Singly, T>;`
* `type DoublyLinkedList<'a, T> = List<'a, Doubly, T>;`

## Time Complexity of Methods

| Method    | Time Complexity |
| -------- | ------- |
| access to front and back of the list  | **O(1)**    |
| push to front and back (`Doubly` only) of the list | **O(1)**     |
| pop from front and back (`Doubly` only) of the list    | **O(1)** |
| insert at an arbitrary position    | O(n)    |
| remove from an arbitrary position    | O(n)    |
| append another list to the front or back of the list    | **O(1)**    |
| retain elements by a predicate    | O(n)    |
| retain and collect remove elements    | O(n)    |
| iteration forwards or backwards (only `Doubly`)    | O(n)    |


## Examples

```rust
use orx_linked_list::*;

fn eq<'a, I: Iterator<Item = &'a u32> + Clone>(iter: I, slice: &[u32]) -> bool {
    iter.clone().count() == slice.len() && iter.zip(slice.iter()).all(|(a, b)| a == b)
}

let _list: List<Singly, u32> = List::new();
let _list = SinglyLinkedList::<u32>::new();
let _list: List<Doubly, u32> = List::new();
let _list = DoublyLinkedList::<u32>::new();

let mut list = DoublyLinkedList::from_iter([3, 4, 5]);
assert_eq!(list.front(), Some(&3));
assert_eq!(list.back(), Some(&5));
assert!(eq(list.iter(), &[3, 4, 5]));
assert!(eq(list.iter_from_back(), &[5, 4, 3]));

assert_eq!(list.pop_front(), Some(3));
assert_eq!(list.pop_back(), Some(5));

list.push_back(5);
list.push_front(3);
assert!(eq(list.iter(), &[3, 4, 5]));

let other = DoublyLinkedList::from_iter([6, 7, 8, 9]);
list.append_back(other);
assert!(eq(list.iter(), &[3, 4, 5, 6, 7, 8, 9]));

let other = DoublyLinkedList::from_iter([0, 1, 2]);
list.append_front(other);
assert!(eq(list.iter(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));

list.retain(&|x| x < &5);
assert!(eq(list.iter(), &[0, 1, 2, 3, 4]));

let mut odds = vec![];
let mut collect_odds = |x| odds.push(x);
list.retain_collect(&|x| x % 2 == 0, &mut collect_odds);

assert!(eq(list.iter(), &[0, 2, 4]));
assert!(eq(odds.iter(), &[1, 3]));
```

## Internal Features

`orx_linked_list::List` makes use of the safety guarantees and efficiency features of [SelfRefCol](https://crates.io/crates/orx-selfref-col).
* `SelfRefCol` constructs its safety guarantees around the fact that all references will be among elements of the same collection. By preventing bringing in external references or leaking out references, it is safe to build the self referential collection with **regular `&` references**.
* With careful encapsulation, `SelfRefCol` prevents passing in external references to the list and leaking within list node references to outside. Once this is established, it provides methods to easily mutate inter list node references. These features allowed a very convenient implementation of the linked list in this crate with almost no use of the `unsafe` keyword, no read or writes through pointers and no access by indices. Compared to the `std::collections::LinkedList` implementation, it can be observed that `orx_linked_list::List` is a much **higher level implementation**.
* Furthermore, `orx_linked_list::List` is **significantly faster** than the standard linked list. One of the main reasons for this is the feature of `SelfRefCol` keeping all close to each other rather than at arbitrary locations in memory which leads to a better cache locality.

## Benchmarks

### Mutation Ends

*You may see the benchmark at [benches/mutation_ends.rs](https://github.com/orxfun/orx-linked-list/blob/main/benches/mutation_ends.rs).*

This benchmark compares time performance of calls to `push_front`, `push_back`, `pop_front` and `pop_back` methods.

<img src="https://raw.githubusercontent.com/orxfun/orx-linked-list/main/docs/img/bench_mutation_ends.PNG" alt="https://raw.githubusercontent.com/orxfun/orx-linked-list/main/docs/img/bench_mutation_ends.PNG" />

### Iteration

*You may see the benchmark at [benches/iter.rs](https://github.com/orxfun/orx-linked-list/blob/main/benches/iter.rs).*

This benchmark compares time performance of iteration through the `iter` method.

<img src="https://raw.githubusercontent.com/orxfun/orx-linked-list/main/docs/img/iter.PNG" alt="https://raw.githubusercontent.com/orxfun/orx-linked-list/main/docs/img/iter.PNG" />

## License

This library is licensed under MIT license. See LICENSE for details.
