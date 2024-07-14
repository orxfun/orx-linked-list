# orx-linked-list

[![orx-linked-list crate](https://img.shields.io/crates/v/orx-linked-list.svg)](https://crates.io/crates/orx-linked-list)
[![orx-linked-list documentation](https://docs.rs/orx-linked-list/badge.svg)](https://docs.rs/orx-linked-list)

An efficient and recursive singly and doubly linked list implementation.

* *efficient*: Please see <a href="#section-benchmarks">benchmarks</a> section for performance reports of common linked list operations. Furthermore, `List` implementation emphasizes safe constant time access and mutations through usage of `NodeIndex`.
* *singly or doubly*: This is a generic parameter of the `List`. As expected, `Doubly` allows for more operations than `Singly`; however, it keeps two references per node rather than one.
* *recursive*: `List` allows creating a list by combining two lists in constant time.


## Variants

* **`List<Variant, T>`** where `T` is the type of elements:
  * **`List<Singly, T>`**: is the singly linked list, where each node holds a reference to the next node.
    * This is equivalent to `List<Singly<MemoryReclaimOnThreshold<2>>, T>`.
    * The alternative memory policy is `List<Singly<MemoryReclaimNever>, T>`.
  * **`List<Doubly, T>`**: is the doubly linked list, where each node holds references to the previous and next nodes.
    * This is equivalent to `List<Doubly<MemoryReclaimOnThreshold<2>>, T>`.
    * The alternative memory policy is `List<Doubly<MemoryReclaimNever>, T>`.

*For possible memory management policies, please see <a href="#section-advanced">advanced usage</a> section.*

## Time Complexity of Methods

In order to indicate the methods available only for the `Doubly` linked list, but not `Singly`, **(*d*)** indicator is used.

The following is the list of methods with constant time **O(1)** time complexity.

| ***O(1)*** Methods |
| -------- |
| **`front`, `back`**: access to front and back of the list  |
| **`get`**: access to to any node with a given index |
| **`push_front`, `push_back`**: push to front or back (*d*) of the list |
| **`pop_front`, `pop_back`**: pop from front and back (*d*) of the list |
| **`insert_prev_to`, `insert_next_to`**: insert a value previous or next to an existing node with a given index (*d*) |
| **`append_front`, `append_back`**: append another list to front or back of the list |
| **`iter`, `iter_from_back`**: create an iterator from the front or back (*d*) of the list; iterating has O(n) time complexity |
| **`iter_forward_from`, `iter_backward_from`**: create a forward or backward (*d*) iterator from any intermediate node with a given index; iterating has O(n) time complexity |

| ***O(n)*** Methods |
| -------- |
| **`index_of`**: get the index of an element, which can later be used for ***O(1)*** methods |
| **`contains`, `position_of`**: check the existence or position of a value |
| **`insert_at`**: insert an element to an arbitrary position of the list |
| **`remove_at`**: remove an element from an arbitrary position of the list |
| **`iter`, `iter_from_back`**: iterate from the front or back (*d*) of the list |
| **`iter_forward_from`, `iter_backward_from`**: iterate in forward or backward (*d*) direction from any intermediate node with a given index |
| **`retain`, `retain_collect`**: retain keeping elements satisfying a predicate and optionally collect removed elements |


## Examples

### Common Usage

`orx_linked_list::List` provides common linked list functionalities, with a special emphasis on maintaining the recursive nature of the data structure which allows for constant time merging of lists.

```rust
use orx_linked_list::*;

fn eq<'a, I: Iterator<Item = &'a u32> + Clone>(iter: I, slice: &[u32]) -> bool {
    iter.clone().count() == slice.len() && iter.zip(slice.iter()).all(|(a, b)| a == b)
}

let _list: List<Singly, u32> = List::new();
let _list: List<Doubly, u32> = List::new();

let mut list = List::<Doubly, _>::from_iter([3, 4, 5]);
assert_eq!(list.front(), Some(&3));
assert_eq!(list.back(), Some(&5));
assert!(eq(list.iter(), &[3, 4, 5]));
assert!(eq(list.iter_from_back(), &[5, 4, 3]));

assert_eq!(list.pop_front(), Some(3));
assert_eq!(list.pop_back(), Some(5));

list.push_back(5);
list.push_front(3);
assert!(eq(list.iter(), &[3, 4, 5]));

let other = List::<Doubly, _>::from_iter([6, 7, 8, 9]);
list.append_back(other);
assert!(eq(list.iter(), &[3, 4, 5, 6, 7, 8, 9]));

let other = List::<Doubly, _>::from_iter([0, 1, 2]);
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

### `NodeIndex` Usage

`NodeIndex` allows indexing into the collection in constant time with safety guarantees. The indices returned by growth methods, such as `push_back` or `append_next_to`, can be stored externally. Otherwise, an index for a value can be searched and obtained in linear time with `index_of` method. You may see below that these indices enable constant time access and mutation methods.

```rust
use orx_linked_list::*;

fn eq<'a, I: Iterator<Item = &'a char> + Clone>(iter: I, slice: &[char]) -> bool {
    iter.clone().count() == slice.len() && iter.zip(slice.iter()).all(|(a, b)| a == b)
}

let mut list = List::<Doubly, _>::from_iter(['a', 'b', 'c', 'd']);

let x = list.index_of(&'x');
assert!(x.is_none());

let maybe_b = list.index_of(&'b'); // O(n)
assert!(maybe_b.is_some());

let b = maybe_b.unwrap();

let data_b = list.get(b); // O(1)
assert_eq!(data_b, Some(&'b'));

// O(1) to create the iterators from the index
assert!(eq(list.iter_forward_from(b).unwrap(), &['b', 'c', 'd']));
assert!(eq(list.iter_backward_from(b).unwrap(), &['b', 'a']));

list.insert_prev_to(b, 'X').unwrap(); // O(1)
list.insert_next_to(b, 'Y').unwrap(); // O(1)
assert!(eq(list.iter(), &['a', 'X', 'b', 'Y', 'c', 'd']));

let removed = list.remove(b); // O(1)
assert_eq!(removed, Ok('b'));
assert!(eq(list.iter(), &['a', 'X', 'Y', 'c', 'd']));

// not possible to wrongly use the index
assert_eq!(list.get(b), None);
assert_eq!(
    list.get_or_error(b).err(),
    Some(NodeIndexError::RemovedNode)
);

// indices can also be stored on insertion
let mut list = List::<Doubly, _>::from_iter(['a', 'b', 'c', 'd']);

let x = list.push_back('x'); // grab index of x in O(1) on insertion

_ = list.push_back('e');
_ = list.push_back('f');
assert!(eq(list.iter(), &['a', 'b', 'c', 'd', 'x', 'e', 'f']));

let data_x = list.get(x); // O(1)
assert_eq!(data_x, Some(&'x'));

list.insert_prev_to(x, 'w').unwrap(); // O(1)
list.insert_next_to(x, 'y').unwrap(); // O(1)
assert!(eq(list.iter(), &['a', 'b', 'c', 'd', 'w', 'x', 'y', 'e', 'f']));
```

<div id="section-advanced"></div>

### Advanced Usage

`NodeIndex` is useful in overcoming the major drawback of linked lists that it requires O(n) time to reach the location to apply the O(1) mutation. With holding the required `NodeIndex`, these mutations can be achieved in O(1). However, in order to use these constant time methods, the node index must be valid.

There are three possible reasons why a node index can be invalid and using it in relevant methods returns `NodeIndexError`:
- **a.** We are using the `NodeIndex` on a different `List`.
- **b.** We are using the `NodeIndex` while the corresponding element is removed from the `List`.
- **c.** `List` executed a memory reclaim under the hood in order to improve memory utilization.

Notice that **a** and **b** are obviously mistakes, and hence, receiving an error is straightforward. Actually, we can see that using a `NodeIndex` on a `List` is much safer than using a `usize` on a `Vec`, as we are not protected against these mistakes in standard vector.

However, **c** is completely related with underlying memory management of the `List`. There are two available policies, which are set as the generic argument of both `Singly` and `Doubly`:
* `MemoryReclaimOnThreshold<D>`
* `MemoryReclaimNever`

#### Default Policy: `MemoryReclaimOnThreshold<2>`

Adding elements to the `List` leads the underlying storage to grow, as expected. On the other hand, removing elements from the list leaves holes in the corresponding positions. In other words, the values are taken out but the memory location where the value is taken out is not immediately used for new elements.

The `MemoryReclaimOnThreshold<D>` policy automatically reclaims these holes whenever the utilization falls below a threshold. The threshold is a function of the constant generic parameter `D`. Specifically, memory of closed nodes will be reclaimed whenever the ratio of closed nodes to all nodes exceeds one over `2^D`.
* when `D = 0`: memory will be reclaimed when utilization is below 0.00% (equivalent to never).
* when `D = 1`: memory will be reclaimed when utilization is below 50.00%.
* when `D = 2`: memory will be reclaimed when utilization is below 75.00%.
* when `D = 3`: memory will be reclaimed when utilization is below 87.50%.
* ...

Underlying `PinnedVec` does not reallocate on memory reclaim operations. Instead, it efficiently moves around the elements within already claimed memory to fill the gaps and repairs the links among the nodes. However, since the positions of the elements will be moved, already obtained `NodeIndex`es might potentially be pointing to wrong positions.
* Fortunately, `NodeIndex` is aware of this operation. Therefore, it is **not** possible to wrongly use the index. If we obtain a node index, then the list reclaims memory, and then we try to use this node index on this list, we receive `NodeIndexError::ReorganizedCollection`.
* Unfortunately, the `NodeIndex` now is not useful. All we can do is re-obtain the index by a linear search with methods such as `index_of`.

```rust
use orx_linked_list::*;

fn float_eq(x: f32, y: f32) -> bool {
    (x - y).abs() < f32::EPSILON
}

// MemoryReclaimOnThreshold<2> -> memory will be reclaimed when utilization is below 75%
let mut list = List::<Doubly, _>::new();
let a = list.push_back('a');
list.push_back('b');
list.push_back('c');
list.push_back('d');
list.push_back('e');

assert!(float_eq(list.node_utilization(), 1.00)); // utilization = 5/5 = 100%

// no reorganization; 'a' is still valid
assert_eq!(list.get_or_error(a), Ok(&'a'));
assert_eq!(list.get(a), Some(&'a'));

_ = list.pop_back(); // leaves a hole

assert!(float_eq(list.node_utilization(), 0.80)); // utilization = 4/5 = 80%

// no reorganization; 'a' is still valid
assert_eq!(list.get_or_error(a), Ok(&'a'));
assert_eq!(list.get(a), Some(&'a'));

_ = list.pop_back(); // leaves the second hole; we have utilization = 3/5 = 60%
                      // this is below the threshold 75%, and triggers reclaim
                      // we claim the two unused nodes / holes

assert!(float_eq(list.node_utilization(), 1.00)); // utilization = 3/3 = 100%

// nodes reorganized; 'a' is no more valid
assert_eq!(
    list.get_or_error(a),
    Err(NodeIndexError::ReorganizedCollection)
);
assert_eq!(list.get(a), None);

// re-obtain the index
let a = list.index_of(&'a').unwrap();
assert_eq!(list.get_or_error(a), Ok(&'a'));
assert_eq!(list.get(a), Some(&'a'));
```

#### Alternative Policy: `MemoryReclaimNever`

However, it is possible to make sure that the node indices will always be valid, unless we manually invalidate them, by simply eliminating the case **c**. Setting the memory reclaim policy to `MemoryReclaimNever` guarantees that there will be no automatic or implicit memory reorganizations:
* use `List<Singly<MemoryReclaimNever>, T>` instead of `List<Singly, T>`, or
* use `List<Doubly<MemoryReclaimNever>, T>` instead of `List<Doubly, T>`.

The drawback of this approach is that memory utilization can be low if there is a large number of pop or remove operations. However, `List` gives caller the control to manage memory by the following two methods:
* `List::node_utilization(&self) -> f32` method can be used to see the ratio of number of active/utilized nodes to the number of used nodes. The caller can decide when to take action by the following.
* `List::reclaim_closed_nodes(&mut self)` method can be used to manually run memory reclaim operation which will bring `node_utilization` to 100% while invalidating already created node indices.

```rust
use orx_linked_list::*;

fn float_eq(x: f32, y: f32) -> bool {
    (x - y).abs() < f32::EPSILON
}

// MemoryReclaimNever -> memory will never be reclaimed automatically
let mut list = List::<Doubly<MemoryReclaimNever>, _>::new();
let a = list.push_back('a');
list.push_back('b');
list.push_back('c');
list.push_back('d');
list.push_back('e');

assert!(float_eq(list.node_utilization(), 1.00)); // utilization = 5/5 = 100%

// no reorganization; 'a' is still valid
assert_eq!(list.get_or_error(a), Ok(&'a'));
assert_eq!(list.get(a), Some(&'a'));

_ = list.pop_back(); // leaves a hole
_ = list.pop_back(); // leaves the second hole
_ = list.pop_back(); // leaves the third hole

assert!(float_eq(list.node_utilization(), 0.40)); // utilization = 2/5 = 40%

// still no reorganization; 'a' is and will always be valid unless we manually reclaim
assert_eq!(list.get_or_error(a), Ok(&'a'));
assert_eq!(list.get(a), Some(&'a'));

list.reclaim_closed_nodes();

// we can manually reclaim memory any time we want to maximize utilization
assert!(float_eq(list.node_utilization(), 1.00)); // utilization = 2/2 = 100%

// we are still protected by list & index validation
// nodes reorganized; 'a' is no more valid, we cannot wrongly use the index
assert_eq!(
    list.get_or_error(a),
    Err(NodeIndexError::ReorganizedCollection)
);
assert_eq!(list.get(a), None);

// re-obtain the index
let a = list.index_of(&'a').unwrap();
assert_eq!(list.get_or_error(a), Ok(&'a'));
assert_eq!(list.get(a), Some(&'a'));
```

## Internal Features

`orx_linked_list::List` makes use of the safety guarantees and efficiency features of [SelfRefCol](https://crates.io/crates/orx-selfref-col).
* `SelfRefCol` constructs its safety guarantees around the fact that all references will be among elements of the same collection. By preventing bringing in external references or leaking out references, it is safe to build the self referential collection with **regular `&` references**.
* With careful encapsulation, `SelfRefCol` prevents passing in external references to the list and leaking within list node references to outside. Once this is established, it provides methods to easily mutate inter list node references. These features allowed a very convenient implementation of the linked list in this crate with almost no use of the `unsafe` keyword, no read or writes through pointers and no access by indices. Compared to the `std::collections::LinkedList` implementation, it can be observed that `orx_linked_list::List` is a much **higher level implementation**.
* Furthermore, `orx_linked_list::List` is **significantly faster** than the standard linked list. One of the main reasons for this is the feature of `SelfRefCol` keeping all close to each other rather than at arbitrary locations in memory which leads to a better cache locality.

<div id="section-benchmarks"></div>

## Benchmarks

### Mutation Ends

*You may see the benchmark at [benches/mutation_ends.rs](https://github.com/orxfun/orx-linked-list/blob/main/benches/mutation_ends.rs).*

This benchmark compares time performance of calls to `push_front`, `push_back`, `pop_front` and `pop_back` methods.

<img src="https://raw.githubusercontent.com/orxfun/orx-linked-list/main/docs/img/bench_mutation_ends.PNG" alt="https://raw.githubusercontent.com/orxfun/orx-linked-list/main/docs/img/bench_mutation_ends.PNG" />

### Iteration

*You may see the benchmark at [benches/iter.rs](https://github.com/orxfun/orx-linked-list/blob/main/benches/iter.rs).*

This benchmark compares time performance of iteration through the `iter` method.

<img src="https://raw.githubusercontent.com/orxfun/orx-linked-list/main/docs/img/iter.PNG" alt="https://raw.githubusercontent.com/orxfun/orx-linked-list/main/docs/img/iter.PNG" />

## Contributing

Contributions are welcome! If you notice an error, have a question or think something could be improved, please open an [issue](https://github.com/orxfun/orx-linked-list/issues/new) or create a PR.

## License

This library is licensed under MIT license. See LICENSE for details.
