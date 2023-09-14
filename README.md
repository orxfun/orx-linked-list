# orx-linked-list

Implements a doubly-linked list.

As opposed to the note of `std::collections::LinkedList`, `orx_linked_list::LinkedList` provides the main theoretical benefits of a linked list; i.e.,
 
* efficient insertion and removal of elements,
 
while aiming to avoid the practical drawbacks related with allocations and CPU cache due to the following:

* `LinkedList` uses an [`ImpVec`](https://crates.io/crates/orx-imp-vec) as the underlying data structure which allows for defining inter-element relationships by thin references; `next` and `prev` relationships are defined by thin `&` references:
    * without any additional heap allocations since smart pointers such as `Box` or `Rc` per element are not necessary;
    * or without the need to use plain index numbers to mimic references.
* All elements are stored in the underlying [`PinnedVec`](https://crates.io/crates/orx-pinned-vec) of the `ImpVec`; which might either be a [`FixedVec`](https://crates.io/crates/orx-fixed-vec) or [`SplitVec`](https://crates.io/crates/orx-split-vec). In either case, the elements will be stored close to each other in a vec-like structure. Although the order of elements in this vector will not be in the correct order as expected in a linked list, they will be pointing to other elements of the same vector. Therefore, unlike classical implementations by arbitrary heap allocations, the `LinkedList` implementation provides better cache locality.

## Usage

Basic usage of the linked list is demonstrated below.

```rust
use orx_linked_list::prelude::*;

let mut list = LinkedList::new();

list.push_back('y');
list.push_front('x');
list.push_back('z');
assert_eq!(vec!['x', 'y', 'z'], list);

assert_eq!(list.pop_back(), Some('z'));
assert_eq!(list.pop_front(), Some('x'));
assert_eq!(vec!['y'], list);

list.push_front('x');
list.push_back('z');
assert_eq!(vec!['x', 'y', 'z'], list);

list.insert_at(1, '?');
assert_eq!(vec!['x', '?', 'y', 'z'], list);

assert_eq!(Some(&'?'), list.get_at(1));
*list.get_mut_at(1).unwrap() = '!';

assert_eq!('!', list.remove_at(1));
assert_eq!(vec!['x', 'y', 'z'], list);
```

## Memory

`LinkedList` provides different levels of control on memory strategies; it can be used by the simplest type signature relying on the defaults or as detailed as possible by determining the tradeoff between time complexity and memory utilization, as well as, the growth stretegies of the backing storage.

### Defaults

The list can be directly used by the default memory management settings by only specifying the element data type.

```rust
use orx_linked_list::prelude::*;

// char list
let mut list = LinkedList::new();
list.push_back('x');

// i32 list with an initial capacity
let mut list = LinkedList::with_initial_capacity(42);
list.push_back(42);
```

Default values of the configurable settings defined in the following subsections are as follows:

* `memory_utilization`: `MemoryUtilization::WithThreshold(0.6)`;
* underlying `PinnedVec`: default `SplitVec` which uses a `Doubling` growth strategy.

### Tradeoff between Memory Utilization & Time Complexity

`LinkedList` holds all elements close to each other in a `PinnedVec` aiming for better cache locality while using thin references rather than wide pointers and to reduce heap allocations. In order to achieve *O(1)* time complexity while avoiding smart pointers, remove and pop operations are designed to be semi-lazy.

In the lazy case; i.e., when the strategy is set to `MemoryReclaimStrategy::Lazy`, every `pop_back`, `pop_front` or `remove_at` operation leaves a gap in the underlying vector. Status of utilization of the underlying vector can be queried using the `memory_status` method and the gaps can completely be reclaimed by manually calling the `memory_reclaim` method which has a time complexity of *O(n)* where *n* is the length of the underlying vector.

Being able to be lazy and to reclaim the gaps, it is possible to define and use different automated strategies which would fit better in different situations: 

* `Lazy`: `memory_reclaim` is never called automatically:
    * leads to the cheapest possible `pop_back`, `pop_front` or `remove_at` operations,
    * however, the utilization of the vector can be low especially when a large number of elements enter and exit the linked list.
    * would be a good fit when it is important keeping the above mentioned operations at as fast as possible; or when utilization is not expected to drop very low.
* `Eager`: every `pop_back`, `pop_front` or `remove_at` method call is automatically followed by a `memory_reclaim` call:
    * this strategy continuously keeps the vector without gaps at 100% utilization;
    * however, abovementioned operations require *O(n)* time complexity;
    * might be a good fit when memory is scarce and more important than the increased time-complexity of these methods.
* `WithThreshold(threshold)`: `pop_back`, `pop_front` or `remove_at` method call is followed by an automatic `memory_reclaim` call only if the memory utilization drops below a pre-determined `threshold`:
    * it is a generalization of `Lazy` and `Eager` allowing to select the required threshold level between memory utilization and amortized time complexity of these methods.

Memory utilization stategy is defined by a field which can be modified any time.

```rust
use orx_linked_list::prelude::*;

let list = LinkedList::<u32>::new()
    .with_memory_utilization(MemoryUtilization::Eager)
    .with_memory_utilization(MemoryUtilization::Lazy)
    .with_memory_utilization(MemoryUtilization::WithThreshold(0.5));
```


### Underlying PinnedVec

`LinkedList` uses an [ImpVec](https://crates.io/crates/orx-imp-vec) to establish and maintain the interconnections among elements of the list. An `ImpVec` can use any vector implementing [PinnedVec](https://crates.io/crates/orx-pinned-vec) which guarantees that the memory locations of elements stay intact. There are two major implementations of `PinnedVec`: a [FixedVec](https://crates.io/crates/orx-fixed-vec) and a [SplitVec](https://crates.io/crates/orx-split-vec). Finally, a split vec can be created with different `Growth` strategies.

This means that a `LinkedList` can use a variety of backing storage. This is apparent from the complete signature without the default type paremeter value:

```rust ignore
LinkedList<'a, T, P> where P: PinnedVec<LinkedListNode<'a, T>>
```

Please see the relevant crates for details; however, below are brief rules of thumbs:

* `FixedVec` has a hard limit on capacity; however, provides complexity and performance of standard vector;
* `SplitVec` allows for a dynamic capacity with different growth strategies:
    * `SplitVec<T, Doubling>`: every time the vector requires new room, a new chunk of memory is allocated which is double the capacity of the prior chunk;
    * `SplitVec<T, Linear>`: every chunk has the same capacity;
    * `SplitVec<T, Exponential>`: this is a generalization of the prior two strategies allowing to define any exponential growth function in between them with a slightly increased access cost.
    * `SplitVec<T, G>`: actually, any growth strategy can be defined by implementing `G: Growth`.

The following type aliases are defined for convenience to simplify the type signatures:

```rust ignore
pub type LinkedListFixed<'a, T>
    = LinkedList<'a, T, FixedVec<LinkedListNode<'a, T>>>;

pub type LinkedListLinear<'a, T>
    = LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, Linear>>;

pub type LinkedListDoubling<'a, T>
    = LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, Doubling>>;

pub type LinkedListExponential<'a, T>
    = LinkedList<'a, T, SplitVec<LinkedListNode<'a, T>, Exponential>>;
```

These variants can be constructed with corresponding associated functions:

```rust
use orx_linked_list::prelude::*;

let list: LinkedListFixed<char> = LinkedList::with_fixed_capacity(100);
let list: LinkedListLinear<char> = LinkedList::with_linear_growth(32);
let list: LinkedListDoubling<char> = LinkedList::with_doubling_growth(4);
let list: LinkedListExponential<char> = LinkedList::with_exponential_growth(4, 1.5);
```

