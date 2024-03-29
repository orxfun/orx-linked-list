use crate::{
    iterators::{backward::IterBackward, forward::IterForward},
    option_utils::some_only_if,
    variants::{doubly::Doubly, ends::ListEnds, list_variant::ListVariant, singly::Singly},
};
use orx_selfref_col::{
    MemoryReclaimOnThreshold, MemoryReclaimPolicy, Node, NodeDataLazyClose, NodeIndex,
    NodeIndexError, NodeRefs, Reclaim, SelfRefCol, SelfRefColMut, Variant,
};
use orx_split_vec::{Recursive, SplitVec};

pub(crate) type DefaultMemoryPolicy = MemoryReclaimOnThreshold<2>;

/// A singly linked [`List`] allowing pushing and popping elements at the front in constant time.
pub type SinglyLinkedList<'a, T, M = DefaultMemoryPolicy> = List<'a, Singly<M>, T>;

/// A doubly linked [`List`] allowing pushing and popping elements both at the front and the back in constant time.
pub type DoublyLinkedList<'a, T, M = DefaultMemoryPolicy> = List<'a, Doubly<M>, T>;

type MutKey<'rf, 'a, V, T> =
    orx_selfref_col::SelfRefColMut<'rf, 'a, V, T, SplitVec<Node<'a, V, T>, Recursive>>;
type DoublyMutKey<'rf, 'a, T, M> = MutKey<'rf, 'a, Doubly<M>, T>;
type SinglyMutKey<'rf, 'a, T, M> = MutKey<'rf, 'a, Singly<M>, T>;

/// Core structure for singly and doubly linked lists:
/// * `type SinglyLinkedList<'a, T> = List<'a, Singly, T>;`
/// * `type DoublyLinkedList<'a, T> = List<'a, Doubly, T>;`
///
/// # Examples
///
/// Below is the simple usage of a doubly linked list.
///
/// ```rust
/// use orx_linked_list::*;
///
/// // empty
/// let doubly = DoublyLinkedList::<i32>::new();
/// let doubly = List::<Doubly, i32>::new();
///
/// // from iter
/// let doubly: DoublyLinkedList<_> = [1, 2, 3].into_iter().collect();
/// let mut doubly: List<Doubly, _> = [1, 2, 3].into_iter().collect();
//
/// assert_eq!(Some(&1), doubly.front());
/// assert_eq!(Some(&3), doubly.back());
///
/// doubly.push_front(0);
/// doubly.push_back(4);
///
/// assert_eq!(Some(&0), doubly.front());
/// assert_eq!(Some(&4), doubly.back());
///
/// assert_eq!(Some(0), doubly.pop_front());
/// assert_eq!(Some(4), doubly.pop_back());
///
/// assert_eq!(vec![1, 2, 3], doubly.iter().copied().collect::<Vec<_>>());
/// assert_eq!(vec![3, 2, 1], doubly.iter_from_back().copied().collect::<Vec<_>>());
/// ````
///
/// Using a singly linked list can be used instead by using the `SinglyLinkedList` type alias or changing the variant from `Doubly` to `Singly`.
///
/// ```rust
/// use orx_linked_list::*;
///
/// // empty
/// let singly = SinglyLinkedList::<i32>::new();
/// let singly = List::<Singly, i32>::new();
///
/// // from iter
/// let singly: SinglyLinkedList<_> = [1, 2, 3].into_iter().collect();
/// let mut singly: List<Singly, _> = [1, 2, 3].into_iter().collect();
//
/// assert_eq!(Some(&1), singly.front());
/// assert_eq!(Some(&3), singly.back());
///
/// singly.push_front(0);
///
/// assert_eq!(Some(&0), singly.front());
///
/// assert_eq!(Some(0), singly.pop_front());
///
/// assert_eq!(vec![1, 2, 3], singly.iter().copied().collect::<Vec<_>>());
/// ````
pub struct List<'a, V, T>
where
    T: 'a,
    V: ListVariant<'a, T>,
    V::Ends: ListEnds<'a, V, T>,
{
    pub(crate) col: SelfRefCol<'a, V, T, SplitVec<Node<'a, V, T>, Recursive>>,
}

impl<'a, V, T> Default for List<'a, V, T>
where
    V: ListVariant<'a, T>,
    V::Ends: ListEnds<'a, V, T>,
{
    /// Creates an empty list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let list: List<Singly, char> = List::default();
    /// assert!(list.is_empty());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, V, T: 'a> List<'a, V, T>
where
    V: ListVariant<'a, T>,
    V::Ends: ListEnds<'a, V, T>,
{
    /// Creates an empty list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let list: List<Singly, char> = List::new();
    /// assert!(list.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            col: SelfRefCol::default(),
        }
    }

    // get
    /// Returns the number of elements in the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: List<Doubly, char> = List::new();
    ///
    /// assert_eq!(0, list.len());
    ///
    /// list.push_back('a');
    /// list.push_front('b');
    /// _ = list.pop_back();
    /// list.push_back('c');
    ///
    /// assert_eq!(2, list.len());
    /// ```
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.col.len()
    }

    /// Returns the number of elements in the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Doubly, _>::new();
    ///
    /// assert!(list.is_empty());
    ///
    /// list.push_back('a');
    /// assert!(!list.is_empty());
    ///
    /// _ = list.pop_back();
    /// assert!(list.is_empty());
    /// ```
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.col.is_empty()
    }

    /// ***O(1)*** Returns a reference to the front of the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Singly, _>::new();
    ///
    /// assert!(list.front().is_none());
    ///
    /// list.push_front('a');
    /// assert_eq!(Some(&'a'), list.front());
    ///
    /// list.push_front('b');
    /// assert_eq!(Some(&'b'), list.front());
    ///
    /// _ = list.pop_front();
    /// assert_eq!(Some(&'a'), list.front());
    /// ```
    #[allow(clippy::missing_panics_doc)]
    pub fn front(&self) -> Option<&T> {
        self.col.ends().front().map(|n| n.data().expect("is-some"))
    }

    /// ***O(1)*** Returns a reference to the back of the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Doubly, _>::new();
    ///
    /// assert!(list.back().is_none());
    ///
    /// list.push_back('a');
    /// assert_eq!(Some(&'a'), list.back());
    ///
    /// list.push_back('b');
    /// assert_eq!(Some(&'b'), list.back());
    ///
    /// list.push_front('c');
    /// assert_eq!(Some(&'b'), list.back());
    ///
    /// _ = list.pop_back();
    /// assert_eq!(Some(&'a'), list.back());
    /// ```
    #[allow(clippy::missing_panics_doc)]
    pub fn back(&self) -> Option<&T> {
        self.col.ends().back().map(|n| n.data().expect("is-some"))
    }

    /// ***O(n)*** Returns an iterator to elements of the list from the `front` node to the back.
    ///
    /// Time complexity:
    /// * ***O(1)*** to access the front node.
    /// * ***O(n)*** to iterate forward from the given node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Doubly, _>::new();
    ///
    /// list.push_front('b');
    /// list.push_back('c');
    /// list.push_front('a');
    ///
    /// let mut iter = list.iter();
    ///
    /// assert_eq!(Some(&'a'), iter.next());
    /// assert_eq!(Some(&'b'), iter.next());
    /// assert_eq!(Some(&'c'), iter.next());
    /// assert!(iter.next().is_none());
    /// ```
    pub fn iter(&self) -> IterForward<'_, 'a, V, T> {
        IterForward::new(self.col.ends().front())
    }

    /// ***O(n)*** Creates a forward iterator starting from the node with the given `node_index`.
    ///
    /// Returns the corresponding `NodeIndexError` if the given index is invalid for this linked list.
    ///
    /// Time complexity:
    /// * ***O(1)*** to access the front node.
    /// * ***O(n)*** to iterate forward from the given node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Doubly, _>::new();
    ///
    /// let b = list.push_front('b');
    /// list.push_back('c');
    /// list.push_front('a');
    /// list.push_back('d'); // list: [a-b-c-d]
    ///
    /// let mut iter = list.iter_forward_from(b);
    ///
    /// assert!(iter.is_ok());
    ///
    /// let mut iter = iter.unwrap();
    ///
    /// assert_eq!(Some(&'b'), iter.next());
    /// assert_eq!(Some(&'c'), iter.next());
    /// assert_eq!(Some(&'d'), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    pub fn iter_forward_from(
        &self,
        node_index: NodeIndex<'a, V, T>,
    ) -> Result<IterForward<'_, 'a, V, T>, NodeIndexError> {
        match node_index.invalidity_reason_for_collection(&self.col) {
            None => Ok(IterForward::new(Some(unsafe {
                node_index.as_ref_unchecked()
            }))),
            Some(e) => Err(e),
        }
    }

    // mut
    /// Clears the list removing all elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Doubly, _>::new();
    ///
    /// list.push_front('b');
    /// list.push_back('c');
    /// list.push_front('a');
    ///
    /// assert_eq!(3, list.len());
    /// assert_eq!(&['a', 'b', 'c'], list.iter().copied().collect::<Vec<_>>().as_slice());
    ///
    /// list.clear();
    /// assert!(list.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.col.clear();
    }

    /// ***O(1)*** Sets value of `front` of the list as `new_front` and returns value of the front element; returns None if list was empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: List<Doubly, _> = List::new();
    ///
    /// assert_eq!(0, list.len());
    ///
    /// let prior_front = list.swap_front('a');
    /// assert!(prior_front.is_none());
    /// assert_eq!(Some(&'a'), list.front());
    ///
    /// let prior_front = list.swap_front('z');
    /// assert_eq!(Some('a'), prior_front);
    /// assert_eq!(Some(&'z'), list.front());
    /// ```
    pub fn swap_front(&mut self, new_front: T) -> Option<T> {
        self.col
            .mutate_take(new_front, |x, value| match x.ends().front() {
                Some(front_node) => Some(front_node.swap_data(&x, value)),
                None => {
                    Self::push_first_node(&x, value);
                    None
                }
            })
    }

    /// ***O(1)*** Sets value of `back` of the list as `new_back` and returns value of the back element; returns None if list was empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: List<Doubly, _> = List::new();
    ///
    /// assert_eq!(0, list.len());
    ///
    /// let prior_back = list.swap_back('a');
    /// assert!(prior_back.is_none());
    /// assert_eq!(Some(&'a'), list.back());
    ///
    /// let prior_back = list.swap_back('z');
    /// assert_eq!(Some('a'), prior_back);
    /// assert_eq!(Some(&'z'), list.back());
    /// ```
    pub fn swap_back(&mut self, new_back: T) -> Option<T> {
        self.col
            .mutate_take(new_back, |x, value| match x.ends().back() {
                Some(back_node) => Some(back_node.swap_data(&x, value)),
                None => {
                    Self::push_first_node(&x, value);
                    None
                }
            })
    }

    /// ***O(1)*** Pops and returns the `value` at the `front` of the list; returns None if the list is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: List<Singly, char> = List::new();
    ///
    /// let popped = list.pop_front();
    /// assert!(popped.is_none());
    ///
    /// list.push_front('a');
    /// assert_eq!(Some(&'a'), list.front());
    ///
    /// let popped = list.pop_front();
    /// assert_eq!(Some('a'), popped);
    /// assert!(list.is_empty());
    /// ```
    pub fn pop_front(&mut self) -> Option<T>
    where
        for<'rf> SelfRefColMut<'rf, 'a, V, T, SplitVec<Node<'a, V, T>, Recursive>>:
            Reclaim<V::Prev, V::Next>,
    {
        self.col.mutate_take((), |x, _| Self::pop_front_node(&x))
    }

    // index
    /// ***O(1)*** Returns a reference to the node with the given `node_index` in constant time.
    ///
    /// Returns None if the index is invalid.
    ///
    /// # Safety
    ///
    /// `get(node_index)` returns `Some` if all of of the following safety and correctness conditions hold:
    /// * the index is created from this linked list,
    /// * the node that this index is created for still belongs to the list`; i.e., it is not removed,
    /// * the node positions in this list are not reorganized to reclaim memory:
    ///   * this case is never observed if `MemoryReclaimNever` is used,
    ///   * this case is observed when:
    ///     * the memory reclaim policy is `MemoryReclaimOnThreshold`, and
    ///     * the utilization of active nodes has dropped a threshold due to pop and remove operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Doubly, _>::new();
    ///
    /// let a = list.push_back('a');
    /// let b = list.push_back('b');
    ///
    /// assert_eq!(Some(&'a'), list.get(a));
    /// assert_eq!(Some(&'b'), list.get(b));
    ///
    /// list.push_front('c');
    /// list.push_back('d');
    /// list.push_front('e');
    /// list.push_back('f');
    ///
    /// assert_eq!(Some(&'a'), list.get(a));
    /// assert_eq!(Some(&'b'), list.get(b));
    ///
    /// list.clear();
    ///
    /// assert_eq!(None, list.get(a));
    /// assert_eq!(None, list.get(b));
    /// ```
    #[inline]
    pub fn get(&self, node_index: NodeIndex<'a, V, T>) -> Option<&T> {
        node_index.data(&self.col)
    }

    /// ***O(1)*** Returns a reference to the node with the given `node_index` in constant time.
    ///
    /// Returns None if the index is invalid.
    ///
    /// # Safety
    ///
    /// `get(node_index)` returns `Some` if all of of the following safety and correctness conditions hold:
    /// * the index is created from this linked list,
    /// * the node that this index is created for still belongs to the list`; i.e., it is not removed,
    /// * the node positions in this list are not reorganized to reclaim memory:
    ///   * this case is never observed if `MemoryReclaimNever` is used,
    ///   * this case is observed when:
    ///     * the memory reclaim policy is `MemoryReclaimOnThreshold`, and
    ///     * the utilization of active nodes has dropped a threshold due to pop and remove operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Doubly, _>::new();
    ///
    /// let a = list.push_back('a');
    /// let b = list.push_back('b');
    ///
    /// assert_eq!(Ok(&'a'), list.get_or_error(a));
    /// assert_eq!(Ok(&'b'), list.get_or_error(b));
    ///
    /// list.push_back('c');
    /// list.push_back('d');
    /// list.push_back('e');
    /// list.push_back('f');
    ///
    /// assert_eq!(
    ///     vec!['a', 'b', 'c', 'd', 'e', 'f'],
    ///     list.iter().copied().collect::<Vec<_>>()
    /// );
    ///
    /// assert_eq!(Ok(&'a'), list.get_or_error(a));
    /// assert_eq!(Ok(&'b'), list.get_or_error(b));
    ///
    /// let removed = list.remove(a);
    /// assert_eq!(removed, Ok('a'));
    ///
    /// assert_eq!(Err(NodeIndexError::RemovedNode), list.get_or_error(a));
    /// assert_eq!(Ok(&'b'), list.get_or_error(b));
    /// ```
    #[inline]
    pub fn get_or_error(&self, node_index: NodeIndex<'a, V, T>) -> Result<&T, NodeIndexError> {
        node_index.data_or_error(&self.col)
    }

    /// ***O(n)*** Performs a forward search from the front and returns the index of the first node with value equal to the given `value`.
    ///
    /// Returns None if there is no element with the given value.
    ///
    /// Obtained `NodeIndex` can later be used for constant time access to the corresponding element.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    /// use orx_selfref_col::NodeIndexError;
    ///
    /// let mut list = List::<Doubly, _>::from_iter(['a', 'b', 'c', 'd']);
    ///
    /// let x = list.index_of(&'x');
    /// assert!(x.is_none());
    ///
    /// let b = list.index_of(&'b'); // O(n)
    /// assert!(b.is_some());
    ///
    /// let b = b.unwrap();
    ///
    /// let data_b = list.get(b); // O(1)
    /// assert_eq!(data_b, Some(&'b'));
    ///
    /// // O(1) to create the iterators from the index
    /// assert_eq!(&['b', 'c', 'd'], list.iter_forward_from(b).unwrap().copied().collect::<Vec<_>>().as_slice());
    /// assert_eq!(&['b', 'a'], list.iter_backward_from(b).unwrap().copied().collect::<Vec<_>>().as_slice());
    ///
    /// list.insert_prev_to(b, 'X').unwrap(); // O(1)
    /// list.insert_next_to(b, 'Y').unwrap(); // O(1)
    /// assert_eq!(&['a', 'X', 'b', 'Y', 'c', 'd'], list.iter().copied().collect::<Vec<_>>().as_slice());
    ///
    /// let removed = list.remove(b);  // O(1)
    /// assert_eq!(removed, Ok('b'));
    /// assert_eq!(&['a', 'X', 'Y', 'c', 'd'], list.iter().copied().collect::<Vec<_>>().as_slice());
    ///
    /// assert_eq!(list.get(b), None);
    /// assert_eq!(list.get_or_error(b).err(), Some(NodeIndexError::RemovedNode));
    /// ```
    pub fn index_of(&self, value: &T) -> Option<NodeIndex<'a, V, T>>
    where
        T: PartialEq,
    {
        self.col.visit_take(value, |x, value| {
            let mut current = x.ends().front();
            while let Some(node) = current {
                match node.data() {
                    Some(data) if value == data => return Some(node.index(&x)),
                    _ => current = *node.next().get(),
                }
            }
            None
        })
    }

    /// ***O(n)*** Performs a forward search from the front and returns `true` if there exists a node with value equal to the given `value`.
    ///
    /// Returns false if there is no element with the given value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Doubly, _>::from_iter(['a', 'b', 'c', 'd']);
    ///
    /// assert!(list.contains(&'a'));
    /// assert!(!list.contains(&'x'));
    /// ```
    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.index_of(value).is_some()
    }

    /// ***O(n)*** Performs a forward search from the front and returns the position of the first node with value equal to the given `value`.
    ///
    /// Returns None if there is no element with the given value.
    ///
    /// Note that, unlike vectors, the position of an element does **not** provide a constant time access to the element.
    /// In order to obtain a `NodeIndex` allowing ***O(1)*** access, use the `index_of` method;
    /// or store the node index returned when the element is added to the list for the first time by methods such as `push_front` or `insert_next_to`, etc.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    /// use orx_selfref_col::NodeIndexError;
    ///
    /// let mut list = List::<Doubly, _>::from_iter(['a', 'b', 'c', 'd']);
    ///
    /// let x = list.position_of(&'x');
    /// assert_eq!(x, None);
    ///
    /// let b = list.position_of(&'b'); // O(n)
    /// assert_eq!(b, Some(1));
    /// ```
    pub fn position_of(&self, value: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        self.iter()
            .enumerate()
            .find(|(_, x)| *x == value)
            .map(|(i, _)| i)
    }

    // memory
    /// Returns the node utilization as a fraction of active nodes to the used nodes:
    /// * 1.0 when there is no closed node;
    /// * 0.0 when all used memory is used by closed nodes.
    ///
    /// Node utilization can be brought to 100%:
    /// * automatically by the underlying memory policy if `MemoryReclaimOnThreshold` is used; or
    /// * manually by calling the `reclaim_closed_nodes` method.
    ///
    /// It is important to note that, memory reclaim operation leads to reorganization of the nodes, which invalidates the node indices obtained before the process.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// fn float_eq(x: f32, y: f32) -> bool {
    ///     (x - y).abs() < f32::EPSILON
    /// }
    ///
    /// // MemoryReclaimOnThreshold<2> -> memory will be reclaimed when utilization is below 75%
    /// let mut list = List::<Doubly, _>::new();
    /// let a = list.push_back('a');
    /// list.push_back('b');
    /// list.push_back('c');
    /// list.push_back('d');
    /// list.push_back('e');
    ///
    /// assert!(float_eq(list.node_utilization(), 1.00)); // utilization = 5/5 = 100%
    ///
    /// // no reorganization; 'a' is still valid
    /// assert_eq!(list.get_or_error(a), Ok(&'a'));
    /// assert_eq!(list.get(a), Some(&'a'));
    ///
    /// _ = list.pop_back(); // leaves a hole
    ///
    /// assert!(float_eq(list.node_utilization(), 0.80)); // utilization = 4/5 = 80%
    ///
    /// // no reorganization; 'a' is still valid
    /// assert_eq!(list.get_or_error(a), Ok(&'a'));
    /// assert_eq!(list.get(a), Some(&'a'));
    ///
    /// _ = list.pop_back(); // leaves the second hole; we have utilization = 3/5 = 60%
    ///                         // this is below the threshold 75%, and triggers reclaim
    ///                         // we claim the two unused nodes / holes
    ///
    /// assert!(float_eq(list.node_utilization(), 1.00)); // utilization = 3/3 = 100%
    ///
    /// // nodes reorganized; 'a' is no more valid
    /// assert_eq!(
    ///     list.get_or_error(a),
    ///     Err(NodeIndexError::ReorganizedCollection)
    /// );
    /// assert_eq!(list.get(a), None);
    /// ```
    #[inline]
    pub fn node_utilization(&self) -> f32 {
        self.col.node_utilization()
    }

    /// Manually attempts to reclaim closed nodes.
    ///
    /// # Safety
    ///
    /// It is important to note that, memory reclaim operation leads to reorganization of the nodes, which invalidates the node indices obtained before the process.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// fn float_eq(x: f32, y: f32) -> bool {
    ///     (x - y).abs() < f32::EPSILON
    /// }
    ///
    /// // MemoryReclaimNever -> memory will never be reclaimed automatically
    /// let mut list = List::<Doubly<MemoryReclaimNever>, _>::new();
    /// let a = list.push_back('a');
    /// list.push_back('b');
    /// list.push_back('c');
    /// list.push_back('d');
    /// list.push_back('e');
    ///
    /// assert!(float_eq(list.node_utilization(), 1.00)); // utilization = 5/5 = 100%
    ///
    /// // no reorganization; 'a' is still valid
    /// assert_eq!(list.get_or_error(a), Ok(&'a'));
    /// assert_eq!(list.get(a), Some(&'a'));
    ///
    /// _ = list.pop_back(); // leaves a hole
    /// _ = list.pop_back(); // leaves the second hole
    /// _ = list.pop_back(); // leaves the third hole
    ///
    /// assert!(float_eq(list.node_utilization(), 0.40)); // utilization = 2/5 = 40%
    ///
    /// // still no reorganization; 'a' is and will always be valid unless we manually reclaim
    /// assert_eq!(list.get_or_error(a), Ok(&'a'));
    /// assert_eq!(list.get(a), Some(&'a'));
    ///
    /// list.reclaim_closed_nodes();
    ///
    /// // we can manually reclaim memory any time we want to maximize utilization
    /// assert!(float_eq(list.node_utilization(), 1.00)); // utilization = 2/2 = 100%
    ///
    /// // we are still protected by list & index validation
    /// // nodes reorganized; 'a' is no more valid, we cannot wrongly use the index
    /// assert_eq!(
    ///     list.get_or_error(a),
    ///     Err(NodeIndexError::ReorganizedCollection)
    /// );
    /// assert_eq!(list.get(a), None);
    /// ```
    pub fn reclaim_closed_nodes(&mut self)
    where
        for<'rf> SelfRefColMut<'rf, 'a, V, T, SplitVec<Node<'a, V, T>, Recursive>>:
            Reclaim<V::Prev, V::Next>,
    {
        self.col.reclaim_closed_nodes()
    }

    // helpers
    /// Pushes the `value` as the first node of the list and sets both ends to this first node.
    #[inline(always)]
    fn push_first_node<'rf>(mut_key: &MutKey<'rf, 'a, V, T>, value: T) -> NodeIndex<'a, V, T> {
        debug_assert!(
            mut_key.is_empty()
                && mut_key.ends().front().is_none()
                && mut_key.ends().back().is_none()
        );
        let node = mut_key.push_get_ref(value);
        mut_key.set_ends([Some(node), Some(node)]);
        node.index(mut_key)
    }

    /// Returns a node reference to the front node of the collection provided that the collection is non-empty.
    ///
    /// # Panics
    ///
    /// Panics if the collection is empty.
    fn get_existing_front<'rf>(mut_key: &MutKey<'rf, 'a, V, T>) -> &'rf Node<'a, V, T> {
        mut_key
            .ends()
            .front()
            .expect("list is nonempty -> front and back exist")
    }

    /// Returns a node reference to the back node of the collection provided that the collection is non-empty.
    ///
    /// # Panics
    ///
    /// Panics if the collection is empty.
    fn get_existing_back<'rf>(mut_key: &MutKey<'rf, 'a, V, T>) -> &'rf Node<'a, V, T> {
        mut_key
            .ends()
            .back()
            .expect("list is nonempty -> front and back exist")
    }

    /// Pops the front node and returns its `value`; returns None if the list is empty.
    fn pop_front_node<'rf>(x: &MutKey<'rf, 'a, V, T>) -> Option<T>
    where
        V: Variant<'a, T, Storage = NodeDataLazyClose<T>>,
        SelfRefColMut<'rf, 'a, V, T, SplitVec<Node<'a, V, T>, Recursive>>:
            Reclaim<V::Prev, V::Next>,
    {
        x.ends().front().map(|prior_front| {
            let new_front = *prior_front.next().get();
            let new_back = some_only_if(new_front.is_some(), x.ends().back());
            x.set_ends([new_front, new_back]);

            if let Some(new_front) = new_front {
                new_front.clear_prev(x);
            }

            prior_front.close_node_take_data(x)
        })
    }
}

impl<'a, T: 'a, M> List<'a, Singly<M>, T>
where
    M: MemoryReclaimPolicy,
{
    // mut

    /// ***O(1)*** Pushes the `value` to the `front` of the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: List<Singly, char> = List::new();
    ///
    /// list.push_front('a');
    /// assert_eq!(Some(&'a'), list.front());
    ///
    /// list.push_front('b');
    /// assert_eq!(Some(&'b'), list.front());
    ///
    /// let popped = list.pop_front();
    /// assert_eq!(Some('b'), popped);
    /// assert_eq!(Some(&'a'), list.front());
    /// ```
    pub fn push_front(&mut self, value: T) -> NodeIndex<'a, Singly<M>, T> {
        self.col
            .mutate_take(value, |x, value| match x.ends().front() {
                Some(prior_front) => {
                    let new_front = x.push_get_ref(value);
                    new_front.set_next(&x, prior_front);
                    x.set_ends([Some(new_front), x.ends().back()]);
                    new_front.index(&x)
                }
                None => Self::push_first_node(&x, value),
            })
    }

    /// ***O(1)*** Appends the `other` list to the `front` of this list.
    ///
    /// Time complexity:
    /// * ***O(1)*** gets `front` of this list, say a,
    /// * ***O(1)*** gets `back` of the other list, say b,
    /// * ***O(1)*** connects `b -> a`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: List<Singly, char> = List::new();
    /// list.push_front('c');
    /// list.push_front('b');
    /// list.push_front('a');
    ///
    /// let mut other: List<Singly, char> = List::new();
    /// other.push_front('e');
    /// other.push_front('d');
    ///
    /// list.append_front(other);
    /// assert_eq!(&['d', 'e', 'a', 'b', 'c'], list.iter().copied().collect::<Vec<_>>().as_slice());
    /// ```
    pub fn append_front(&mut self, other: Self) {
        self.col.append_mutate(other.col, (), |x, y, _| {
            match (x.ends().front(), y.ends().back()) {
                (Some(a), Some(b)) => {
                    b.set_next(&x, a);
                    x.set_ends([y.ends().front(), x.ends().back()]);
                }
                (None, Some(_)) => {
                    x.set_ends([y.ends().front(), y.ends().back()]);
                }
                _ => {}
            };
        });
    }

    /// ***O(1)*** Appends the `other` list to the `back` of this list.
    ///
    /// Time complexity:
    /// * ***O(1)*** gets `back` of this list, say a,
    /// * ***O(1)*** gets `front` of the other list, say b,
    /// * ***O(1)*** connects `a -> b`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: List<Singly, char> = List::new();
    /// list.push_front('c');
    /// list.push_front('b');
    /// list.push_front('a');
    ///
    /// let mut other: List<Singly, char> = List::new();
    /// other.push_front('e');
    /// other.push_front('d');
    ///
    /// list.append_back(other);
    /// assert_eq!(&['a', 'b', 'c', 'd', 'e'], list.iter().copied().collect::<Vec<_>>().as_slice());
    /// ```
    pub fn append_back(&mut self, other: Self) {
        self.col.append_mutate(other.col, (), |x, y, _| {
            match (x.ends().back(), y.ends().front()) {
                (Some(a), Some(b)) => {
                    a.set_next(&x, b);
                    x.set_ends([x.ends().front(), y.ends().back()]);
                }
                (None, Some(b)) => {
                    x.set_ends([Some(b), y.ends().back()]);
                }
                _ => {}
            };
        });
    }

    /// ***O(n)*** Removes and returns value of the `at`-th element in the list; returns None if list length is less than or equal to `at`.
    ///
    /// Time complexity:
    /// * starts from the `front`,
    /// * ***O(n)*** iterates until reaching the element,
    /// * ***O(1)*** removes and returns the value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Singly, char>::new();
    ///
    /// list.push_front('c');
    /// list.push_front('b');
    /// list.push_front('a');
    /// assert_eq!(&['a', 'b', 'c'], list.iter().copied().collect::<Vec<_>>().as_slice());
    ///
    /// let removed = list.remove_at(3);
    /// assert!(removed.is_none());
    /// assert_eq!(&['a', 'b', 'c'], list.iter().copied().collect::<Vec<_>>().as_slice());
    ///
    /// let removed = list.remove_at(1);
    /// assert_eq!(Some('b'), removed);
    /// assert_eq!(&['a', 'c'], list.iter().copied().collect::<Vec<_>>().as_slice());
    ///```
    pub fn remove_at(&mut self, at: usize) -> Option<T> {
        match at {
            _ if at >= self.len() => None,
            0 => self.pop_front(),
            _ => self.col.mutate_take(at, |x, at| {
                let (prev, current) = Self::get_prev_and_current_at(&x, at);
                prev.set_next(&x, *current.next().get());
                if at == x.len() - 1 {
                    x.set_ends([x.ends().front(), Some(prev)]);
                }
                Some(current.close_node_take_data(&x))
            }),
        }
    }

    /// ***O(n)*** Inserts the given `value` at the `at`-th element of the list.
    ///
    /// Time complexity:
    /// * starts from the `front`,
    /// * ***O(n)*** iterates until reaching the element,
    /// * ***O(1)*** inserts the value.
    ///
    /// # Panics
    ///
    /// Panics if `at > self.len()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Singly, char>::new();
    ///
    /// list.push_front('c');
    /// list.push_front('b');
    /// list.push_front('a');
    /// assert_eq!(&['a', 'b', 'c'], list.iter().copied().collect::<Vec<_>>().as_slice());
    ///
    /// list.insert_at(1, 'x');
    /// assert_eq!(&['a', 'x', 'b', 'c'], list.iter().copied().collect::<Vec<_>>().as_slice());
    ///```
    pub fn insert_at(&mut self, at: usize, value: T) -> NodeIndex<'a, Singly<M>, T> {
        assert!(at <= self.len(), "out of bounds");
        match at {
            0 => self.push_front(value),
            at if at == self.len() => self.col.mutate_take((at, value), |x, (at, value)| {
                let new_node = x.push_get_ref(value);
                x.set_ends([x.ends().front(), Some(new_node)]);
                let (_, prev) = Self::get_prev_and_current_at(&x, at - 1);
                prev.set_next(&x, new_node);
                new_node.index(&x)
            }),
            at => self.col.mutate_take((at, value), |x, (at, value)| {
                let new_node = x.push_get_ref(value);
                let (prev, current) = Self::get_prev_and_current_at(&x, at);
                prev.set_next(&x, new_node);
                new_node.set_next(&x, current);
                new_node.index(&x)
            }),
        }
    }

    /// ***O(n)*** Retains only the elements specified by the predicate.
    ///
    /// In other words, removes all elements `e` for which `predicate(&e)` returns false.
    /// This method operates in place, visiting each element exactly once in the original order, and preserves the order of the retained elements.
    ///
    /// Time complexity:
    /// * ***O(n)*** to iterate over all elements,
    ///   * ***O(1)*** to remove elements failing to satisfy the predicate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Singly, _>::from_iter([0, 1, 2, 3, 4]);
    ///
    /// list.retain(&|x| *x % 2 == 0);
    ///
    /// assert_eq!(&[0, 2, 4], list.iter().copied().collect::<Vec<_>>().as_slice());
    /// ```
    pub fn retain<Predicate>(&mut self, predicate: &Predicate)
    where
        Predicate: Fn(&T) -> bool,
    {
        self.retain_collect(predicate, &mut |_| {});
    }

    /// ***O(n)*** Retains only the elements specified by the predicate; all elements that are removed elements are collected by the provided closure.
    ///
    /// In other words, removes all elements `e` for which `predicate(&e)` returns false; and calls `collect(e)` on the removed values.
    /// This method operates in place, visiting each element exactly once in the original order, and preserves the order of the retained elements.
    ///
    /// Time complexity:
    /// * ***O(n)*** to iterate over all elements,
    ///   * ***O(1)*** to remove elements failing to satisfy the predicate and collect.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Singly, _>::from_iter([0, 1, 2, 3, 4]);
    ///
    /// let mut odds = vec![];
    /// let mut collect = |x| odds.push(x);
    ///
    /// list.retain_collect(&|x| *x % 2 == 0, &mut collect);
    ///
    /// assert_eq!(&[0, 2, 4], list.iter().copied().collect::<Vec<_>>().as_slice());
    /// assert_eq!(&[1, 3], odds.as_slice());
    /// ```
    pub fn retain_collect<Predicate, Collect>(
        &mut self,
        predicate: &Predicate,
        collect: &mut Collect,
    ) where
        Predicate: Fn(&T) -> bool,
        Collect: FnMut(T),
    {
        fn remove<'a, T, M>(
            mut_key: &SinglyMutKey<'_, 'a, T, M>,
            prev: Option<&'a Node<'a, Singly<M>, T>>,
            node_to_remove: &'a Node<'a, Singly<M>, T>,
        ) -> T
        where
            M: MemoryReclaimPolicy,
        {
            if let Some(prev) = prev {
                prev.set_next(mut_key, *node_to_remove.next().get());
            }
            node_to_remove.close_node_take_data_no_reclaim(mut_key)
        }

        self.col
            .mutate_filter_collect(predicate, collect, |x, predicate, collect| {
                let mut front = x.ends().front();
                let mut back = None;
                let mut back_dropped = false;
                let mut current = x.ends().front();

                if let Some(node) = current {
                    current = *node.next().get();
                    if predicate(unsafe { node.data().unwrap_unchecked() }) {
                        back = Some(node);
                        back_dropped = false;
                    } else {
                        collect(remove(&x, back, node));
                        front = None;
                        back_dropped = true;
                    }
                }

                while let Some(node) = current {
                    current = *node.next().get();
                    if predicate(unsafe { node.data().unwrap_unchecked() }) {
                        back = Some(node);
                        back_dropped = false;
                        if front.is_none() {
                            front = Some(node);
                        }
                    } else {
                        collect(remove(&x, back, node));
                        back_dropped = true;
                    }
                }

                if !back_dropped {
                    back = x.ends().back();
                }

                x.set_ends([front, back]);
            });
        self.col.reclaim_closed_nodes();
    }

    // helpers
    /// ***O(n)*** Gets the prev -> current node tuple where current is the `at`-th element.
    ///
    /// # Panics
    ///
    /// Panics if `self.len() < 2` and/or `at == 0`.
    #[allow(clippy::type_complexity)]
    fn get_prev_and_current_at<'rf>(
        mut_key: &SinglyMutKey<'rf, 'a, T, M>,
        at: usize,
    ) -> (&'a Node<'a, Singly<M>, T>, &'a Node<'a, Singly<M>, T>) {
        let mut prev = unsafe { mut_key.ends().front().unwrap_unchecked() };
        let mut current = unsafe { prev.next().get().unwrap_unchecked() };
        for _ in 1..at {
            prev = current;
            current = unsafe { current.next().get().unwrap_unchecked() };
        }

        (prev, current)
    }
}

impl<'a, T: 'a, M> List<'a, Doubly<M>, T>
where
    M: MemoryReclaimPolicy,
{
    // get
    /// ***O(n)*** Performs a backward search from the back and returns the index of the first node with value equal to the given `value`.
    ///
    /// Returns None if there is no element with the given value.
    ///
    /// Obtained `NodeIndex` can later be used for constant time access to the corresponding element.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    /// use orx_selfref_col::NodeIndexError;
    ///
    /// let mut list = List::<Doubly, _>::from_iter(['a', 'b', 'c', 'd']);
    ///
    /// let x = list.index_of_from_back(&'x');
    /// assert!(x.is_none());
    ///
    /// let b = list.index_of_from_back(&'b'); // O(n)
    /// assert!(b.is_some());
    ///
    /// let b = b.unwrap();
    ///
    /// let data_b = list.get(b); // O(1)
    /// assert_eq!(data_b, Some(&'b'));
    ///
    /// // O(1) to create the iterators from the index
    /// assert_eq!(&['b', 'c', 'd'], list.iter_forward_from(b).unwrap().copied().collect::<Vec<_>>().as_slice());
    /// assert_eq!(&['b', 'a'], list.iter_backward_from(b).unwrap().copied().collect::<Vec<_>>().as_slice());
    ///
    /// list.insert_prev_to(b, 'X').unwrap(); // O(1)
    /// list.insert_next_to(b, 'Y').unwrap(); // O(1)
    /// assert_eq!(&['a', 'X', 'b', 'Y', 'c', 'd'], list.iter().copied().collect::<Vec<_>>().as_slice());
    ///
    /// let removed = list.remove(b);  // O(1)
    /// assert_eq!(removed, Ok('b'));
    /// assert_eq!(&['a', 'X', 'Y', 'c', 'd'], list.iter().copied().collect::<Vec<_>>().as_slice());
    ///
    /// assert_eq!(list.get(b), None);
    /// assert_eq!(list.get_or_error(b).err(), Some(NodeIndexError::RemovedNode));
    /// ```
    pub fn index_of_from_back(&self, value: &T) -> Option<NodeIndex<'a, Doubly<M>, T>>
    where
        T: PartialEq,
    {
        self.col.visit_take(value, |x, value| {
            let mut current = x.ends().back();
            while let Some(node) = current {
                match node.data() {
                    Some(data) if value == data => return Some(node.index(&x)),
                    _ => current = *node.prev().get(),
                }
            }
            None
        })
    }

    /// ***O(n)*** Performs a backward search from the back and returns `true` if there exists a node with value equal to the given `value`.
    ///
    /// Returns false if there is no element with the given value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Doubly, _>::from_iter(['a', 'b', 'c', 'd']);
    ///
    /// assert!(list.contains_from_back(&'a'));
    /// assert!(!list.contains_from_back(&'x'));
    /// ```
    pub fn contains_from_back(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.index_of_from_back(value).is_some()
    }

    /// ***O(n)*** Returns an iterator to elements of the list from the `back` node to the front.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Doubly, _>::new();
    ///
    /// list.push_front('b');
    /// list.push_back('c');
    /// list.push_front('a');
    ///
    /// let mut iter = list.iter_from_back();
    ///
    /// assert_eq!(Some(&'c'), iter.next());
    /// assert_eq!(Some(&'b'), iter.next());
    /// assert_eq!(Some(&'a'), iter.next());
    /// assert!(iter.next().is_none());
    /// ```
    pub fn iter_from_back(&self) -> IterBackward<'_, 'a, Doubly<M>, T> {
        IterBackward::new(self.col.ends().back())
    }

    /// ***O(n)*** Creates a backward iterator starting from the node with the given `node_index`.
    ///
    /// Returns the corresponding `NodeIndexError` if the given index is invalid for this linked list.
    ///
    /// Time complexity:
    /// * ***O(1)*** to access the front node.
    /// * ***O(n)*** to iterate forward from the given node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Doubly, _>::new();
    ///
    /// let b = list.push_front('b');
    /// list.push_back('c');
    /// list.push_front('a');
    /// list.push_back('d'); // list: [a-b-c-d]
    ///
    /// let mut iter = list.iter_backward_from(b);
    ///
    /// assert!(iter.is_ok());
    ///
    /// let mut iter = iter.unwrap();
    ///
    /// assert_eq!(Some(&'b'), iter.next());
    /// assert_eq!(Some(&'a'), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    pub fn iter_backward_from(
        &self,
        node_index: NodeIndex<'a, Doubly<M>, T>,
    ) -> Result<IterBackward<'_, 'a, Doubly<M>, T>, NodeIndexError> {
        match node_index.invalidity_reason_for_collection(&self.col) {
            None => Ok(IterBackward::new(Some(unsafe {
                node_index.as_ref_unchecked()
            }))),
            Some(e) => Err(e),
        }
    }

    /// ***O(1)*** Pushes the `value` to the `front` of the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: List<Doubly, char> = List::new();
    ///
    /// list.push_front('a');
    /// assert_eq!(Some(&'a'), list.front());
    ///
    /// list.push_front('b');
    /// assert_eq!(Some(&'b'), list.front());
    ///
    /// let popped = list.pop_front();
    /// assert_eq!(Some('b'), popped);
    /// assert_eq!(Some(&'a'), list.front());
    /// ```
    pub fn push_front(&mut self, value: T) -> NodeIndex<'a, Doubly<M>, T> {
        self.col
            .mutate_take(value, |x, value| Self::push_front_node(&x, value))
    }

    /// ***O(1)*** Pushes the `value` to the `back` of the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: List<Doubly, char> = List::new();
    ///
    /// list.push_back('a');
    /// assert_eq!(Some(&'a'), list.back());
    ///
    /// list.push_back('b');
    /// assert_eq!(Some(&'b'), list.back());
    ///
    /// let popped = list.pop_back();
    /// assert_eq!(Some('b'), popped);
    /// assert_eq!(Some(&'a'), list.back());
    /// ```
    pub fn push_back(&mut self, value: T) -> NodeIndex<'a, Doubly<M>, T> {
        self.col
            .mutate_take(value, |x, value| Self::push_back_node(&x, value))
    }

    /// ***O(1)*** Appends the `other` list to the `front` of this list.
    ///
    /// Time complexity:
    /// * ***O(1)*** gets `front` of this list, say a,
    /// * ***O(1)*** gets `back` of the other list, say b,
    /// * ***O(1)*** connects `b -> a`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: List<Doubly, char> = List::new();
    /// list.push_front('b');
    /// list.push_front('a');
    /// list.push_back('c');
    ///
    /// let mut other: List<Doubly, char> = List::new();
    /// other.push_back('d');
    /// other.push_back('e');
    ///
    /// list.append_front(other);
    /// assert_eq!(&['d', 'e', 'a', 'b', 'c'], list.iter().copied().collect::<Vec<_>>().as_slice());
    /// ```
    pub fn append_front(&mut self, other: Self) {
        self.col.append_mutate(other.col, (), |x, y, _| {
            match (x.ends().front(), y.ends().back()) {
                (Some(a), Some(b)) => {
                    b.set_next(&x, a);
                    a.set_prev(&x, b);
                    x.set_ends([y.ends().front(), x.ends().back()]);
                }
                (None, Some(_)) => {
                    x.set_ends([y.ends().front(), y.ends().back()]);
                }
                _ => {}
            };
        });
    }

    /// ***O(1)*** Appends the `other` list to the `back` of this list.
    ///
    /// Time complexity:
    /// * ***O(1)*** gets `back` of this list, say a,
    /// * ***O(1)*** gets `front` of the other list, say b,
    /// * ***O(1)*** connects `a -> b`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: List<Doubly, char> = List::new();
    /// list.push_front('b');
    /// list.push_front('a');
    /// list.push_back('c');
    ///
    /// let mut other: List<Doubly, char> = List::new();
    /// other.push_back('d');
    /// other.push_back('e');
    ///
    /// list.append_back(other);
    /// assert_eq!(&['a', 'b', 'c', 'd', 'e'], list.iter().copied().collect::<Vec<_>>().as_slice());
    /// ```
    pub fn append_back(&mut self, other: Self) {
        self.col.append_mutate(other.col, (), |x, y, _| {
            match (x.ends().back(), y.ends().front()) {
                (Some(a), Some(b)) => {
                    a.set_next(&x, b);
                    b.set_prev(&x, a);
                    x.set_ends([x.ends().front(), y.ends().back()]);
                }
                (None, Some(b)) => {
                    x.set_ends([Some(b), y.ends().back()]);
                }
                _ => {}
            };
        });
    }

    /// ***O(1)*** Pops and returns the `value` at the `back` of the list; returns None if the list is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: List<Doubly, char> = List::new();
    ///
    /// let popped = list.pop_back();
    /// assert!(popped.is_none());
    ///
    /// list.push_back('a');
    /// assert_eq!(Some(&'a'), list.back());
    ///
    /// let popped = list.pop_back();
    /// assert_eq!(Some('a'), popped);
    /// assert!(list.is_empty());
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        self.col.mutate_take((), |x, _| {
            x.ends().back().map(|prior_back| {
                let new_back = *prior_back.prev().get();
                let new_front = some_only_if(new_back.is_some(), x.ends().front());
                x.set_ends([new_front, new_back]);

                if let Some(back) = new_back {
                    back.clear_next(&x);
                }

                prior_back.close_node_take_data(&x)
            })
        })
    }

    /// ***O(n)*** Removes and returns value of the `at`-th element in the list; returns None if list length is less than or equal to `at`.
    ///
    /// Time complexity:
    /// * starts from the `front` or `back` choosing the shorter path depending on the length of the list and value of `at`,
    /// * ***O(n)*** iterates until reaching the element,
    /// * ***O(1)*** removes and returns the value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Doubly, char>::new();
    ///
    /// list.push_front('b');
    /// list.push_back('c');
    /// list.push_front('a');
    /// assert_eq!(&['a', 'b', 'c'], list.iter().copied().collect::<Vec<_>>().as_slice());
    ///
    /// let removed = list.remove_at(3);
    /// assert!(removed.is_none());
    /// assert_eq!(&['a', 'b', 'c'], list.iter().copied().collect::<Vec<_>>().as_slice());
    ///
    /// let removed = list.remove_at(1);
    /// assert_eq!(Some('b'), removed);
    /// assert_eq!(&['a', 'c'], list.iter().copied().collect::<Vec<_>>().as_slice());
    ///```
    pub fn remove_at(&mut self, at: usize) -> Option<T> {
        match at {
            _ if at >= self.len() => None,
            0 => self.pop_front(),
            _ if at == self.len() - 1 => self.pop_back(),
            _ => {
                let at_from_back = self.len() - 1 - at;
                if at <= at_from_back {
                    self.col.mutate_take(at, |x, at| {
                        let current = Self::get_node_at(&x, at);
                        Some(Self::remove_node(&x, current))
                    })
                } else {
                    self.col.mutate_take(at_from_back, |x, at_from_back| {
                        let current = Self::get_node_at_from_back(&x, at_from_back);
                        Some(Self::remove_node(&x, current))
                    })
                }
            }
        }
    }

    /// ***O(1)*** Removes and returns value of the element with the given `node_index`.
    ///
    /// Does not change the list and returns the `NodeIndexError` if the node index is invalid.
    ///
    /// # Safety
    ///
    /// Removal is carried out only if the `node_index` is valid.
    /// And the node index is valid if all of of the following safety and correctness conditions hold:
    /// * the index is created from this linked list,
    /// * the node that this index is created for still belongs to the list`; i.e., it is not removed,
    /// * the node positions in this list are not reorganized to reclaim memory:
    ///   * this case is never observed if `MemoryReclaimNever` is used,
    ///   * this case is observed when:
    ///     * the memory reclaim policy is `MemoryReclaimOnThreshold`, and
    ///     * the utilization of active nodes has dropped a threshold due to pop and remove operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    /// use orx_selfref_col::NodeIndexError;
    ///
    /// let mut list = List::<Doubly, _>::new();
    ///
    /// list.push_back('a');
    /// let b = list.push_back('b');
    /// list.push_back('c');
    /// list.push_back('d');
    ///
    /// assert_eq!(vec!['a', 'b', 'c', 'd'], list.iter().copied().collect::<Vec<_>>());
    ///
    /// let removed = list.remove(b);
    ///
    /// assert_eq!(removed, Ok('b'));
    /// assert_eq!(vec!['a', 'c', 'd'], list.iter().copied().collect::<Vec<_>>());
    ///
    /// let not_removed = list.remove(b);
    /// assert_eq!(not_removed.err(), Some(NodeIndexError::RemovedNode));
    /// assert_eq!(vec!['a', 'c', 'd'], list.iter().copied().collect::<Vec<_>>());
    ///```
    #[allow(clippy::missing_panics_doc)]
    pub fn remove(&mut self, node_index: NodeIndex<'a, Doubly<M>, T>) -> Result<T, NodeIndexError> {
        self.col.mutate_take(node_index, |x, idx| {
            x.get_node_ref_or_error(idx).map(|node| {
                if node.ref_eq(Self::get_existing_front(&x)) {
                    Self::pop_front_node(&x).expect("node exists")
                } else if node.ref_eq(Self::get_existing_back(&x)) {
                    Self::pop_back_node(&x).expect("node exists")
                } else {
                    Self::remove_node(&x, node)
                }
            })
        })
    }

    /// ***O(n)*** Inserts the given `value` at the `at`-th element of the list.
    ///
    /// Time complexity:
    /// * starts from the `front` or `back` choosing the shorter path depending on the length of the list and value of `at`,
    /// * ***O(n)*** iterates until reaching the element,
    /// * ***O(1)*** inserts the value.
    ///
    /// # Panics
    ///
    /// Panics if `at > self.len()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Doubly, char>::new();
    ///
    /// list.push_back('c');
    /// list.push_front('b');
    /// list.push_front('a');
    /// assert_eq!(&['a', 'b', 'c'], list.iter().copied().collect::<Vec<_>>().as_slice());
    ///
    /// list.insert_at(1, 'x');
    /// assert_eq!(&['a', 'x', 'b', 'c'], list.iter().copied().collect::<Vec<_>>().as_slice());
    ///```
    pub fn insert_at(&mut self, at: usize, value: T) -> NodeIndex<'a, Doubly<M>, T> {
        assert!(at <= self.len(), "out of bounds");
        match at {
            0 => self.push_front(value),
            _ if at == self.len() => self.push_back(value),
            at => {
                let at_from_back = self.len() - 1 - at;
                if at <= at_from_back {
                    self.col.mutate_take((at, value), |x, (at, value)| {
                        let current = Self::get_node_at(&x, at);
                        Self::insert_node_prev_to(&x, current, value)
                    })
                } else {
                    self.col
                        .mutate_take((at_from_back, value), |x, (at_from_back, value)| {
                            let current = Self::get_node_at_from_back(&x, at_from_back);
                            Self::insert_node_prev_to(&x, current, value)
                        })
                }
            }
        }
    }

    /// ***O(1)*** Inserts the given `value` as the previous of the node with the given `node_index` and returns the index of the inserted node.
    ///
    /// Does not change the list and returns the `NodeIndexError` if the node index is invalid.
    ///
    /// # Safety
    ///
    /// Insertion is carried out only if the `node_index` is valid.
    /// And the node index is valid if all of of the following safety and correctness conditions hold:
    /// * the index is created from this linked list,
    /// * the node that this index is created for still belongs to the list`; i.e., it is not removed,
    /// * the node positions in this list are not reorganized to reclaim memory:
    ///   * this case is never observed if `MemoryReclaimNever` is used,
    ///   * this case is observed when:
    ///     * the memory reclaim policy is `MemoryReclaimOnThreshold`, and
    ///     * the utilization of active nodes has dropped a threshold due to pop and remove operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Doubly, _>::new();
    ///
    /// list.push_back('a');
    /// let b = list.push_back('b');
    /// list.push_back('c');
    /// list.push_back('d');
    ///
    /// assert_eq!(vec!['a', 'b', 'c', 'd'], list.iter().copied().collect::<Vec<_>>());
    ///
    /// let x = list.insert_prev_to(b, 'X').unwrap();
    ///
    /// assert_eq!(list.get(x), Some(&'X'));
    /// assert_eq!(vec!['a', 'X', 'b', 'c', 'd'], list.iter().copied().collect::<Vec<_>>());
    ///```
    pub fn insert_prev_to(
        &mut self,
        node_index: NodeIndex<'a, Doubly<M>, T>,
        value: T,
    ) -> Result<NodeIndex<'a, Doubly<M>, T>, NodeIndexError> {
        self.col
            .mutate_take((node_index, value), |x, (idx, value)| {
                x.get_node_ref_or_error(idx).map(|node| {
                    let front = Self::get_existing_front(&x);
                    if node.ref_eq(front) {
                        Self::push_front_node(&x, value)
                    } else {
                        Self::insert_node_prev_to(&x, node, value)
                    }
                })
            })
    }

    /// ***O(1)*** Inserts the given `value` as the next of the node with the given `node_index` and returns the index of the inserted node.
    ///
    /// Does not change the list and returns the `NodeIndexError` if the node index is invalid.
    ///
    /// # Safety
    ///
    /// Insertion is carried out only if the `node_index` is valid.
    /// And the node index is valid if all of of the following safety and correctness conditions hold:
    /// * the index is created from this linked list,
    /// * the node that this index is created for still belongs to the list`; i.e., it is not removed,
    /// * the node positions in this list are not reorganized to reclaim memory:
    ///   * this case is never observed if `MemoryReclaimNever` is used,
    ///   * this case is observed when:
    ///     * the memory reclaim policy is `MemoryReclaimOnThreshold`, and
    ///     * the utilization of active nodes has dropped a threshold due to pop and remove operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Doubly, _>::new();
    ///
    /// list.push_back('a');
    /// let b = list.push_back('b');
    /// list.push_back('c');
    /// list.push_back('d');
    ///
    /// assert_eq!(vec!['a', 'b', 'c', 'd'], list.iter().copied().collect::<Vec<_>>());
    ///
    /// let x = list.insert_next_to(b, 'X').unwrap();
    ///
    /// assert_eq!(list.get(x), Some(&'X'));
    /// assert_eq!(vec!['a', 'b', 'X', 'c', 'd'], list.iter().copied().collect::<Vec<_>>());
    ///```
    pub fn insert_next_to(
        &mut self,
        node_index: NodeIndex<'a, Doubly<M>, T>,
        value: T,
    ) -> Result<NodeIndex<'a, Doubly<M>, T>, NodeIndexError> {
        self.col
            .mutate_take((node_index, value), |x, (idx, value)| {
                x.get_node_ref_or_error(idx).map(|node| {
                    let back = Self::get_existing_back(&x);
                    if node.ref_eq(back) {
                        Self::push_back_node(&x, value)
                    } else {
                        Self::insert_node_next_to(&x, node, value)
                    }
                })
            })
    }

    /// ***O(n)*** Retains only the elements specified by the predicate.
    ///
    /// In other words, removes all elements `e` for which `predicate(&e)` returns false.
    /// This method operates in place, visiting each element exactly once in the original order, and preserves the order of the retained elements.
    ///
    /// Time complexity:
    /// * ***O(n)*** to iterate over all elements,
    ///   * ***O(1)*** to remove elements failing to satisfy the predicate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Doubly, _>::from_iter([0, 1, 2, 3, 4]);
    ///
    /// list.retain(&|x| *x % 2 == 0);
    ///
    /// assert_eq!(&[0, 2, 4], list.iter().copied().collect::<Vec<_>>().as_slice());
    /// ```
    pub fn retain<Predicate>(&mut self, predicate: &Predicate)
    where
        Predicate: Fn(&T) -> bool,
    {
        self.retain_collect(predicate, &mut |_| {});
    }

    /// ***O(n)*** Retains only the elements specified by the predicate; all elements that are removed elements are collected by the provided closure.
    ///
    /// In other words, removes all elements `e` for which `predicate(&e)` returns false; and calls `collect(e)` on the removed values.
    /// This method operates in place, visiting each element exactly once in the original order, and preserves the order of the retained elements.
    ///
    /// Time complexity:
    /// * ***O(n)*** to iterate over all elements,
    ///   * ***O(1)*** to remove elements failing to satisfy the predicate and collect.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = List::<Doubly, _>::from_iter([0, 1, 2, 3, 4]);
    ///
    /// let mut odds = vec![];
    /// let mut collect = |x| odds.push(x);
    ///
    /// list.retain_collect(&|x| *x % 2 == 0, &mut collect);
    ///
    /// assert_eq!(&[0, 2, 4], list.iter().copied().collect::<Vec<_>>().as_slice());
    /// assert_eq!(&[1, 3], odds.as_slice());
    /// ```
    pub fn retain_collect<Predicate, Collect>(
        &mut self,
        predicate: &Predicate,
        collect: &mut Collect,
    ) where
        Predicate: Fn(&T) -> bool,
        Collect: FnMut(T),
    {
        fn remove<'a, T, M>(
            mut_key: &DoublyMutKey<'_, 'a, T, M>,
            node: &'a Node<'a, Doubly<M>, T>,
        ) -> T
        where
            M: MemoryReclaimPolicy,
        {
            if let Some(next) = node.next().get() {
                next.set_prev(mut_key, *node.prev().get());
            }

            if let Some(prev) = node.prev().get() {
                prev.set_next(mut_key, *node.next().get());
            }

            node.close_node_take_data_no_reclaim(mut_key)
        }

        self.col
            .mutate_filter_collect(predicate, collect, |x, predicate, collect| {
                let mut front = x.ends().front();
                let mut back = None;
                let mut back_dropped = false;
                let mut current = x.ends().front();

                if let Some(node) = current {
                    current = *node.next().get();
                    if predicate(unsafe { node.data().unwrap_unchecked() }) {
                        back = Some(node);
                        back_dropped = false;
                    } else {
                        collect(remove(&x, node));
                        front = None;
                        back_dropped = true;
                    }
                }

                while let Some(node) = current {
                    current = *node.next().get();
                    if predicate(unsafe { node.data().unwrap_unchecked() }) {
                        back = Some(node);
                        back_dropped = false;
                        if front.is_none() {
                            front = Some(node);
                        }
                    } else {
                        collect(remove(&x, node));
                        back_dropped = true;
                    }
                }

                if !back_dropped {
                    back = x.ends().back();
                }

                x.set_ends([front, back]);
            });
        self.col.reclaim_closed_nodes();
    }

    // helpers
    /// Removes the `node` from the list, repairs the links and returns the removed value.
    fn remove_node<'rf>(
        mut_key: &DoublyMutKey<'rf, 'a, T, M>,
        node: &'a Node<'a, Doubly<M>, T>,
    ) -> T {
        if let Some(next) = node.next().get() {
            next.set_prev(mut_key, *node.prev().get());
        }

        if let Some(prev) = node.prev().get() {
            prev.set_next(mut_key, *node.next().get());
        }

        node.close_node_take_data(mut_key)
    }

    /// Inserts the `new_value` to the list previous to the given `node`.
    ///
    /// Returns the index of the new node created for the `new_value`.
    fn insert_node_prev_to<'rf>(
        x: &DoublyMutKey<'rf, 'a, T, M>,
        node: &'a Node<'a, Doubly<M>, T>,
        new_value: T,
    ) -> NodeIndex<'a, Doubly<M>, T> {
        let new_node = x.push_get_ref(new_value);

        if let Some(prev) = node.prev().get() {
            prev.set_next(x, new_node);
            new_node.set_prev(x, *prev);
        }

        new_node.set_next(x, node);
        node.set_prev(x, new_node);

        new_node.index(x)
    }

    /// Inserts the `new_value` to the list next to the given `node`.
    ///
    /// Returns the index of the new node created for the `new_value`.
    fn insert_node_next_to<'rf>(
        x: &DoublyMutKey<'rf, 'a, T, M>,
        node: &'a Node<'a, Doubly<M>, T>,
        new_value: T,
    ) -> NodeIndex<'a, Doubly<M>, T> {
        let new_node = x.push_get_ref(new_value);

        if let Some(next) = node.next().get() {
            next.set_prev(x, new_node);
            new_node.set_next(x, *next);
        }

        new_node.set_prev(x, node);
        node.set_next(x, new_node);

        new_node.index(x)
    }

    /// ***O(n)*** Gets the node at the `at`-th position.
    ///
    /// # Panics
    ///
    /// Panics if `self.is_empty()`.
    fn get_node_at<'rf>(
        mut_key: &DoublyMutKey<'rf, 'a, T, M>,
        at: usize,
    ) -> &'a Node<'a, Doubly<M>, T> {
        let mut current = unsafe { mut_key.ends().front().unwrap_unchecked() };
        for _ in 0..at {
            current = unsafe { current.next().get().unwrap_unchecked() };
        }
        current
    }

    /// ***O(n)*** Gets the node at the `at_from_back`-th position from the back.
    ///
    /// # Panics
    ///
    /// Panics if `self.is_empty()`.
    fn get_node_at_from_back<'rf>(
        mut_key: &DoublyMutKey<'rf, 'a, T, M>,
        at_from_back: usize,
    ) -> &'a Node<'a, Doubly<M>, T> {
        let mut current = unsafe { mut_key.ends().back().unwrap_unchecked() };
        for _ in 0..at_from_back {
            current = unsafe { current.prev().get().unwrap_unchecked() };
        }
        current
    }

    /// Pushes the `value` as the front node and returns the index to the created node.
    fn push_front_node<'rf>(
        x: &MutKey<'rf, 'a, Doubly<M>, T>,
        value: T,
    ) -> NodeIndex<'a, Doubly<M>, T> {
        match x.ends().front() {
            Some(prior_front) => {
                let new_front = x.push_get_ref(value);
                new_front.set_next(x, prior_front);
                prior_front.set_prev(x, new_front);
                x.set_ends([Some(new_front), x.ends().back()]);
                new_front.index(x)
            }
            None => Self::push_first_node(x, value),
        }
    }

    /// Pushes the `value` as the back node and returns the index to the created node.
    fn push_back_node<'rf>(
        x: &MutKey<'rf, 'a, Doubly<M>, T>,
        value: T,
    ) -> NodeIndex<'a, Doubly<M>, T> {
        match x.ends().back() {
            Some(prior_back) => {
                let new_back = x.push_get_ref(value);
                new_back.set_prev(x, prior_back);
                prior_back.set_next(x, new_back);
                x.set_ends([x.ends().front(), Some(new_back)]);
                new_back.index(x)
            }
            None => Self::push_first_node(x, value),
        }
    }

    /// Pops the back node and returns its `value`; returns None if the list is empty.
    fn pop_back_node<'rf>(x: &MutKey<'rf, 'a, Doubly<M>, T>) -> Option<T> {
        x.ends().back().map(|prior_back| {
            let new_back = *prior_back.prev().get();
            let new_front = some_only_if(new_back.is_some(), x.ends().front());
            x.set_ends([new_front, new_back]);

            if let Some(back) = new_back {
                back.clear_next(x);
            }

            prior_back.close_node_take_data(x)
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use test_case::test_matrix;

    pub(crate) fn assert_empty_list<'a, V, T>(list: &List<'a, V, T>)
    where
        V: ListVariant<'a, T>,
        V::Ends: ListEnds<'a, V, T>,
    {
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert!(list.col.ends().front().is_none());
        assert!(list.col.ends().back().is_none());
        assert!(list.front().is_none());
        assert!(list.back().is_none());
        assert!(list.iter().next().is_none());

        V::validate(list);
    }

    pub(crate) fn validate_both<'a, T>(singly: &List<'a, Singly, T>, doubly: &List<'a, Doubly, T>) {
        Singly::validate(singly);
        Doubly::validate(doubly);
    }

    #[test]
    fn new() {
        let list: List<Singly, usize> = List::new();
        assert_empty_list(&list);

        let list: List<Doubly, usize> = List::new();
        assert_empty_list(&list);
    }

    #[test]
    fn default() {
        let list: List<Singly, usize> = List::default();
        assert_empty_list(&list);

        let list: List<Doubly, usize> = List::default();
        assert_empty_list(&list);
    }

    #[test]
    fn len() {
        let mut singly: List<Singly, _> = List::default();
        let mut doubly: List<Doubly, _> = List::default();

        for i in 0..100 {
            assert_eq!(i, singly.len());
            assert_eq!(i, doubly.len());
            singly.push_front(i);
            doubly.push_front(i);
            validate_both(&singly, &doubly);
        }

        for i in 0..100 {
            assert_eq!(100 - i, singly.len());
            assert_eq!(100 - i, doubly.len());
            singly.pop_front();
            doubly.pop_front();
            validate_both(&singly, &doubly);
        }

        assert_empty_list(&singly);
        assert_empty_list(&doubly);

        for i in 0..100 {
            assert_eq!(i, doubly.len());
            doubly.push_back(i);
            validate_both(&singly, &doubly);
        }

        for i in 0..100 {
            assert_eq!(100 - i, doubly.len());
            doubly.pop_front();
            validate_both(&singly, &doubly);
        }
    }

    #[test]
    fn singly_front() {
        let mut singly: List<Singly, _> = List::default();
        Singly::validate(&singly);

        assert!(singly.front().is_none());

        singly.push_front(42);
        Singly::validate(&singly);

        assert_eq!(Some(&42), singly.front());

        singly.push_front(7);
        Singly::validate(&singly);

        assert_eq!(Some(&7), singly.front());

        singly.pop_front();
        Singly::validate(&singly);

        assert_eq!(Some(&42), singly.front());

        singly.pop_front();
        Singly::validate(&singly);

        assert!(singly.front().is_none());

        assert_empty_list(&singly);
    }

    #[test]
    fn singly_back() {
        let mut singly = SinglyLinkedList::<_, MemoryReclaimOnThreshold<2>>::new();
        assert_empty_list(&singly);

        singly.push_front('x');
        assert_eq!(Some(&'x'), singly.front());
        assert_eq!(Some(&'x'), singly.back());

        let _ = singly.pop_front();
        assert_empty_list(&singly);

        singly.push_front('c');
        singly.push_front('b');
        singly.push_front('a');
        assert_eq!(Some(&'a'), singly.front());
        assert_eq!(Some(&'c'), singly.back());
        Singly::validate(&singly);

        let _ = singly.pop_front();
        assert_eq!(Some(&'b'), singly.front());
        assert_eq!(Some(&'c'), singly.back());
        Singly::validate(&singly);

        let _ = singly.pop_front();
        assert_eq!(Some(&'c'), singly.front());
        assert_eq!(Some(&'c'), singly.back());
        Singly::validate(&singly);

        let _ = singly.pop_front();
        assert_empty_list(&singly);

        singly.push_front('c');
        singly.push_front('b');
        singly.push_front('a');
        assert_eq!(Some(&'c'), singly.back());
        Singly::validate(&singly);

        singly.insert_at(3, 'd');
        assert_eq!(Some(&'d'), singly.back());
        Singly::validate(&singly);

        _ = singly.pop_front();
        assert_eq!(Some(&'d'), singly.back());
        singly.push_front('a');
        assert_eq!(Some(&'d'), singly.back());
        Singly::validate(&singly);

        assert_eq!(
            &['a', 'b', 'c', 'd'],
            singly.iter().copied().collect::<Vec<_>>().as_slice()
        );

        singly.remove_at(3);
        assert_eq!(
            &['a', 'b', 'c'],
            singly.iter().copied().collect::<Vec<_>>().as_slice()
        );
        Singly::validate(&singly);
        assert_eq!(Some(&'c'), singly.back());
    }

    #[test]
    fn doubly_front_back() {
        let mut doubly: List<Doubly, _> = List::default();
        Doubly::validate(&doubly);

        assert!(doubly.front().is_none());
        assert!(doubly.back().is_none());

        doubly.push_front(42);
        Doubly::validate(&doubly);

        assert_eq!(Some(&42), doubly.front());
        assert_eq!(Some(&42), doubly.back());

        doubly.push_front(7);
        Doubly::validate(&doubly);

        assert_eq!(Some(&7), doubly.front());
        assert_eq!(Some(&42), doubly.back());

        doubly.pop_front();
        Doubly::validate(&doubly);

        assert_eq!(Some(&42), doubly.front());
        assert_eq!(Some(&42), doubly.back());

        doubly.push_back(7);
        Doubly::validate(&doubly);

        assert_eq!(Some(&42), doubly.front());
        assert_eq!(Some(&7), doubly.back());

        doubly.pop_back();
        Doubly::validate(&doubly);

        assert_eq!(Some(&42), doubly.front());
        assert_eq!(Some(&42), doubly.back());

        doubly.pop_back();
        Doubly::validate(&doubly);

        assert_empty_list(&doubly);
    }

    #[test]
    fn swap_front_back() {
        let mut doubly: List<Doubly, _> = List::new();
        let mut singly: List<Singly, _> = List::default();
        validate_both(&singly, &doubly);

        fn when_empty<'a, V: ListVariant<'a, i32>>(list: &mut List<'a, V, i32>)
        where
            V::Ends: ListEnds<'a, V, i32>,
        {
            let old = list.swap_front(42);
            assert!(old.is_none());
            assert_eq!(list.front(), Some(&42));
            assert_eq!(list.back(), Some(&42));

            list.clear();

            let old = list.swap_back(42);
            assert!(old.is_none());
            assert_eq!(list.front(), Some(&42));
            assert_eq!(list.back(), Some(&42));
        }
        when_empty(&mut singly);
        when_empty(&mut doubly);
        validate_both(&singly, &doubly);

        fn when_single_item<'a, V: ListVariant<'a, i32>>(
            list: &mut List<'a, V, i32>,
            swap_front: bool,
        ) where
            V::Ends: ListEnds<'a, V, i32>,
        {
            let old = if swap_front {
                list.swap_front(7)
            } else {
                list.swap_back(7)
            };
            assert_eq!(old, Some(42));
            assert_eq!(list.front(), Some(&7));
            assert_eq!(list.back(), Some(&7));
        }

        for swap_front in [true, false] {
            doubly.clear();
            singly.clear();
            doubly.push_front(42);
            singly.push_front(42);
            when_single_item(&mut singly, swap_front);
            when_single_item(&mut doubly, swap_front);
            validate_both(&singly, &doubly);
        }

        fn when_multi_items<'a, V: ListVariant<'a, i32>>(
            list: &mut List<'a, V, i32>,
            swap_front: bool,
        ) where
            V::Ends: ListEnds<'a, V, i32>,
        {
            let prior_front = list.front().copied();
            let prior_back = list.back().copied();

            let (expected, old) = if swap_front {
                (prior_front, list.swap_front(7))
            } else {
                (prior_back, list.swap_back(7))
            };
            assert_eq!(expected, old);

            if swap_front {
                assert_eq!(list.front(), Some(&7));
                assert_eq!(list.back().copied(), prior_back);
            } else {
                assert_eq!(list.front().copied(), prior_front);
                assert_eq!(list.back(), Some(&7));
            }
        }

        for swap_front in [true, false] {
            doubly.clear();
            singly.clear();
            doubly.push_front(0);
            singly.push_front(1);
            doubly.push_front(2);
            singly.push_front(3);
            doubly.push_front(4);
            singly.push_front(5);
            when_multi_items(&mut singly, swap_front);
            when_multi_items(&mut doubly, swap_front);
            validate_both(&singly, &doubly);
        }
    }

    #[test]
    fn remove_at() {
        let n = 2142;
        let indices_to_remove = [2141, 2140, 2000, 1648, 1200, 999, 512, 444, 31, 21, 7, 1, 0];

        let mut doubly: List<Doubly, _> = List::new();
        let mut singly: List<Singly, _> = List::new();

        for i in 0..n {
            doubly.push_back(i);
            singly.push_front(n - 1 - i);
        }

        for i in indices_to_remove {
            let removed = doubly.remove_at(i);
            assert_eq!(Some(i), removed);

            let removed = singly.remove_at(i);
            assert_eq!(Some(i), removed);

            validate_both(&singly, &doubly);
        }
    }

    #[test_matrix([0,1,2,6,58,548,1024,2047,3122,3123,3124])]
    fn insert_at(at: usize) {
        let n = 3124;
        let value = 10000;

        let mut doubly: List<Doubly, _> = List::new();
        let mut singly: List<Singly, _> = List::new();

        for i in 0..n {
            doubly.push_back(i);
            singly.push_front(n - 1 - i);
        }

        doubly.insert_at(at, value);
        singly.insert_at(at, value);

        assert_eq!(Some(&value), doubly.iter().nth(at));
        assert_eq!(Some(&value), singly.iter().nth(at));

        validate_both(&singly, &doubly);
    }

    fn push_pop_clear_singly(n_push: usize, singly: &mut SinglyLinkedList<usize>) {
        for i in 0..n_push {
            singly.push_front(n_push - 1 - i);
        }
        Singly::validate(singly);

        while !singly.is_empty() {
            singly.pop_front();
            Singly::validate(singly);
        }
    }

    fn push_pop_clear_doubly(n_push: usize, doubly: &mut DoublyLinkedList<usize>) {
        for i in 0..n_push {
            doubly.push_front(n_push - 1 - i);
        }
        Doubly::validate(doubly);

        while !doubly.is_empty() {
            doubly.pop_front();
            Doubly::validate(doubly);
        }
    }

    #[test]
    fn remove() {
        let mut wrong_collection = DoublyLinkedList::<_, MemoryReclaimOnThreshold<2>>::new();
        let n = 1000;

        for i in 0..n {
            let mut doubly = DoublyLinkedList::new();
            let mut idx = None;
            for j in 0..n {
                let index = doubly.push_back(j);
                if i == j {
                    idx = Some(index);
                }
            }
            let idx = idx.expect("is some");
            let removed = doubly.remove(idx);
            Doubly::validate(&doubly);
            assert_eq!(removed, Ok(i));

            let list: Vec<_> = doubly.iter().copied().collect();
            if i > 0 {
                for (j, &elem) in list.iter().enumerate().take(i - 1) {
                    assert_eq!(elem, j);
                }
            }
            for (j, &elem) in list.iter().enumerate().skip(i) {
                assert_eq!(elem, j + 1);
            }

            assert_eq!(
                wrong_collection.remove(idx).err(),
                Some(NodeIndexError::WrongCollection)
            );
        }
    }

    #[test]
    fn insert_prev_to() {
        let mut wrong_collection = DoublyLinkedList::<_, MemoryReclaimOnThreshold<2>>::new();
        let n = 1000;

        for i in 0..n {
            let mut doubly = DoublyLinkedList::new();
            let mut idx = None;
            for j in 0..n {
                let index = doubly.push_back(j);
                if i == j {
                    idx = Some(index);
                }
            }
            let idx = idx.expect("is some");
            let new_idx = doubly.insert_prev_to(idx, 10000);
            Doubly::validate(&doubly);
            assert_eq!(doubly.get(new_idx.expect("succeeded")), Some(&10000));

            let list: Vec<_> = doubly.iter().copied().collect();
            for (j, &elem) in list.iter().enumerate().take(i) {
                assert_eq!(elem, j);
            }
            assert_eq!(list[i], 10000);
            for (j, &elem) in list.iter().enumerate().skip(i + 1) {
                assert_eq!(elem, j - 1);
            }

            assert_eq!(
                wrong_collection.insert_prev_to(idx, 10000).err(),
                Some(NodeIndexError::WrongCollection)
            );
        }
    }

    #[test]
    fn insert_next_to() {
        let mut wrong_collection = DoublyLinkedList::<_, MemoryReclaimOnThreshold<2>>::new();
        let n = 1000;

        for i in 0..n {
            let mut doubly = DoublyLinkedList::new();
            let mut idx = None;
            for j in 0..n {
                let index = doubly.push_back(j);
                if i == j {
                    idx = Some(index);
                }
            }
            let idx = idx.expect("is some");
            let new_idx = doubly.insert_next_to(idx, 10000);
            Doubly::validate(&doubly);
            assert_eq!(doubly.get(new_idx.expect("succeeded")), Some(&10000));

            let list: Vec<_> = doubly.iter().copied().collect();
            for (j, &elem) in list.iter().enumerate().take(i + 1) {
                assert_eq!(elem, j);
            }
            assert_eq!(list[i + 1], 10000);
            for (j, &elem) in list.iter().enumerate().skip(i + 2) {
                assert_eq!(elem, j - 1);
            }

            assert_eq!(
                wrong_collection.insert_next_to(idx, 10000).err(),
                Some(NodeIndexError::WrongCollection)
            );
        }
    }

    #[test]
    fn append_empty_empty() {
        let n = 1314;

        let mut singly = SinglyLinkedList::<usize>::new();
        singly.append_back(SinglyLinkedList::new());
        let mut doubly = DoublyLinkedList::<usize>::new();
        doubly.append_back(DoublyLinkedList::new());

        assert_empty_list(&singly);
        assert_empty_list(&doubly);
        push_pop_clear_singly(n, &mut singly);
        push_pop_clear_doubly(n, &mut doubly);

        let mut singly = SinglyLinkedList::<usize>::new();
        singly.append_front(SinglyLinkedList::new());
        let mut doubly = DoublyLinkedList::<usize>::new();
        doubly.append_front(DoublyLinkedList::new());

        assert_empty_list(&singly);
        assert_empty_list(&doubly);
        push_pop_clear_singly(n, &mut singly);
        push_pop_clear_doubly(n, &mut doubly);
    }

    #[test]
    fn append_empty_nonempty() {
        let n = 1572;

        fn validate<'a>(
            n: usize,
            singly: &mut SinglyLinkedList<'a, usize>,
            doubly: &mut DoublyLinkedList<'a, usize>,
        ) {
            assert_eq!(n, singly.len());
            assert_eq!(n, doubly.len());

            for (i, x) in singly.iter().enumerate() {
                assert_eq!(i, *x);
            }
            for (i, x) in doubly.iter().enumerate() {
                assert_eq!(i, *x);
            }

            validate_both(singly, doubly);
            push_pop_clear_singly(n, singly);
            push_pop_clear_doubly(n, doubly);
        }

        let mut singly = SinglyLinkedList::new();
        let mut other = SinglyLinkedList::new();
        for i in 0..n {
            other.push_front(n - 1 - i);
        }
        singly.append_back(other);

        let mut doubly = DoublyLinkedList::new();
        let mut other = DoublyLinkedList::new();
        for i in 0..n {
            other.push_front(n - 1 - i);
        }
        doubly.append_back(other);

        validate(n, &mut singly, &mut doubly);

        let mut singly = SinglyLinkedList::new();
        let mut other = SinglyLinkedList::new();
        for i in 0..n {
            other.push_front(n - 1 - i);
        }
        singly.append_front(other);

        let mut doubly = DoublyLinkedList::new();
        let mut other = DoublyLinkedList::new();
        for i in 0..n {
            other.push_front(n - 1 - i);
        }
        doubly.append_front(other);

        validate(n, &mut singly, &mut doubly);
    }

    #[test]
    fn append_nonempty_empty() {
        fn validate<'a>(
            n: usize,
            singly: &mut SinglyLinkedList<'a, usize>,
            doubly: &mut DoublyLinkedList<'a, usize>,
        ) {
            assert_eq!(n, singly.len());
            assert_eq!(n, doubly.len());

            for (i, x) in singly.iter().enumerate() {
                assert_eq!(i, *x);
            }
            for (i, x) in doubly.iter().enumerate() {
                assert_eq!(i, *x);
            }

            validate_both(singly, doubly);
            push_pop_clear_singly(n, singly);
            push_pop_clear_doubly(n, doubly);
        }

        let n = 1044;

        let mut singly = SinglyLinkedList::new();
        for i in 0..n {
            singly.push_front(n - 1 - i);
        }
        singly.append_back(SinglyLinkedList::new());

        let mut doubly = DoublyLinkedList::new();
        for i in 0..n {
            doubly.push_front(n - 1 - i);
        }
        doubly.append_back(DoublyLinkedList::new());

        validate(n, &mut singly, &mut doubly);
    }

    #[test]
    fn append_back_nonempty_nonempty() {
        let n1 = 1344;
        let n2 = 123;

        let mut singly = SinglyLinkedList::new();
        let mut other = SinglyLinkedList::new();
        for i in 0..n1 {
            other.push_front(n1 + n2 - 1 - i);
        }
        for i in n1..(n1 + n2) {
            singly.push_front(n1 + n2 - 1 - i);
        }
        singly.append_back(other);
        assert_eq!(n1 + n2, singly.len());
        for (i, x) in singly.iter().enumerate() {
            assert_eq!(i, *x);
        }
        Singly::validate(&singly);
        push_pop_clear_singly(n1, &mut singly);

        let mut doubly = DoublyLinkedList::new();
        let mut other = DoublyLinkedList::new();
        for i in 0..n1 {
            other.push_front(n1 + n2 - 1 - i);
        }
        for i in n1..(n1 + n2) {
            doubly.push_front(n1 + n2 - 1 - i);
        }

        doubly.append_back(other);
        assert_eq!(n1 + n2, doubly.len());

        for (i, x) in doubly.iter().enumerate() {
            assert_eq!(i, *x);
        }
        Doubly::validate(&doubly);
        push_pop_clear_doubly(n1, &mut doubly);
    }

    #[test]
    fn append_front_nonempty_nonempty() {
        let n1 = 1344;
        let n2 = 123;

        let mut singly = SinglyLinkedList::new();
        let mut other = SinglyLinkedList::new();
        for i in 0..n1 {
            singly.push_front(n1 + n2 - 1 - i);
        }
        for i in n1..(n1 + n2) {
            other.push_front(n1 + n2 - 1 - i);
        }
        singly.append_front(other);
        assert_eq!(n1 + n2, singly.len());
        for (i, x) in singly.iter().enumerate() {
            assert_eq!(i, *x);
        }
        Singly::validate(&singly);
        push_pop_clear_singly(n1, &mut singly);

        let mut doubly = DoublyLinkedList::new();
        let mut other = DoublyLinkedList::new();
        for i in 0..n1 {
            doubly.push_front(n1 + n2 - 1 - i);
        }
        for i in n1..(n1 + n2) {
            other.push_front(n1 + n2 - 1 - i);
        }

        doubly.append_front(other);
        assert_eq!(n1 + n2, doubly.len());

        for (i, x) in doubly.iter().enumerate() {
            assert_eq!(i, *x);
        }
        Doubly::validate(&doubly);
        push_pop_clear_doubly(n1, &mut doubly);
    }

    #[test_matrix(
        [0, 1, 2, 6, 7, 35],
        [
            vec![],
            vec![0],
            vec![0, 1, 2, 3, 4, 5, 6, 7],
            vec![0, 1, 2, 3, 4, 5, 6],
            vec![1, 2, 3, 4, 5, 6, 7],
        ]
    )]
    fn retain(num_gaps: usize, values: Vec<i32>) {
        for modulo in [Box::new(0), Box::new(1)] {
            let mut doubly = DoublyLinkedList::from_iter(values.clone());
            let mut singly = SinglyLinkedList::from_iter(values.clone());
            for _ in 0..num_gaps {
                singly.push_front(111);
                doubly.push_front(111);
                _ = singly.pop_front();
                _ = doubly.pop_front();
            }

            let predicate = |x: &i32| x % 2 == *modulo;

            let front = values.iter().find(|x| predicate(x));
            let back = values.iter().rev().find(|x| predicate(x));

            doubly.retain(&predicate);
            singly.retain(&predicate);
            validate_both(&singly, &doubly);

            let expected_list = values
                .iter()
                .filter(|x| *x % 2 == *modulo)
                .copied()
                .collect::<Vec<_>>();

            assert_eq!(
                expected_list,
                doubly.iter().copied().collect::<Vec<_>>().as_slice()
            );
            assert_eq!(
                expected_list,
                singly.iter().copied().collect::<Vec<_>>().as_slice()
            );

            assert_eq!(front, doubly.front());
            assert_eq!(back, doubly.back());

            assert_eq!(front, singly.front());
            assert_eq!(back, singly.back());
        }
    }

    #[test_matrix(
        [0, 1, 2, 6, 7, 35],
        [
            vec![],
            vec![0],
            vec![0, 1, 2, 3, 4, 5, 6, 7],
            vec![0, 1, 2, 3, 4, 5, 6],
            vec![1, 2, 3, 4, 5, 6, 7],
        ]
    )]
    fn retain_collect(num_gaps: usize, values: Vec<i32>) {
        for modulo in [Box::new(0), Box::new(1)] {
            let mut doubly = DoublyLinkedList::from_iter(values.clone());
            let mut singly = SinglyLinkedList::from_iter(values.clone());
            for _ in 0..num_gaps {
                singly.push_front(111);
                doubly.push_front(111);
                _ = singly.pop_front();
                _ = doubly.pop_front();
            }

            let predicate = |x: &i32| x % 2 == *modulo;

            let front = values.iter().find(|x| predicate(x));
            let back = values.iter().rev().find(|x| predicate(x));

            let expected_collected = values
                .iter()
                .filter(|x| *x % 2 == (1 - *modulo))
                .copied()
                .collect::<Vec<_>>();

            let mut collected = vec![];
            let mut collect = |x| collected.push(x);
            doubly.retain_collect(&predicate, &mut collect);
            assert_eq!(expected_collected, collected.as_slice());

            let mut collected = vec![];
            let mut collect = |x| collected.push(x);
            singly.retain_collect(&predicate, &mut collect);
            assert_eq!(expected_collected, collected.as_slice());

            validate_both(&singly, &doubly);

            let expected_list = values
                .iter()
                .filter(|x| *x % 2 == *modulo)
                .copied()
                .collect::<Vec<_>>();

            assert_eq!(
                expected_list,
                doubly.iter().copied().collect::<Vec<_>>().as_slice()
            );
            assert_eq!(
                expected_list,
                singly.iter().copied().collect::<Vec<_>>().as_slice()
            );

            assert_eq!(front, doubly.front());
            assert_eq!(back, doubly.back());

            assert_eq!(front, singly.front());
            assert_eq!(back, singly.back());
        }
    }

    #[test]
    fn node_utilization() {
        use crate::*;

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
    }

    #[test]
    fn reclaim_closed_nodes() {
        use crate::*;

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
    }
}
