use crate::{Doubly, DoublyIdx, DoublyIterable, List, Singly, SinglyIdx, SinglyIterable};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node, NodeIdx};

// both

impl<T, M, P> List<Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    T: PartialEq,
    P: PinnedVec<Node<Singly<T>>>,
{
    /// ***O(n)*** Performs a forward search from the front and returns the index of the first node with value equal to the given `value`.
    ///
    /// Returns None if there is no element with the given value.
    ///
    /// Obtained `NodeIdx` can later be used for constant time access to the corresponding element.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::from_iter(['a', 'b', 'c', 'd']);
    ///
    /// let x = list.idx_of(&'x');
    /// assert!(x.is_none());
    ///
    /// let b = list.idx_of(&'b'); // O(n)
    /// assert!(b.is_some());
    ///
    /// let b = b.unwrap();
    ///
    /// let data_b = list.get(b); // O(1)
    /// assert_eq!(data_b, Some(&'b'));
    ///
    /// // O(1) to create the iterators from the index
    /// assert_eq!(&['b', 'c', 'd'], list.iter_from(b).copied().collect::<Vec<_>>().as_slice());
    /// assert_eq!(&['b', 'a'], list.iter_backward_from(b).copied().collect::<Vec<_>>().as_slice());
    ///
    /// list.insert_prev_to(b, 'X'); // O(1)
    /// list.insert_next_to(b, 'Y'); // O(1)
    /// assert!(list.eq_to_iter_vals(['a', 'X', 'b', 'Y', 'c', 'd']));
    ///
    /// let removed = list.remove(b);  // O(1)
    /// assert_eq!(removed, 'b');
    /// assert!(list.eq_to_iter_vals(['a', 'X', 'Y', 'c', 'd']));
    ///
    /// assert_eq!(list.get(b), None);
    /// assert_eq!(list.idx_err(b), Some(NodeIdxError::RemovedNode));
    /// ```
    pub fn idx_of(&self, value: &T) -> Option<SinglyIdx<T>> {
        self.iter_ptr()
            .find(|p| self.0.node(*p).data().is_some_and(|d| d == value))
            .map(|p| NodeIdx::new(self.memory_state(), p))
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
    /// let list: DoublyList<_> = ['a', 'b', 'c', 'd'].into_iter().collect();
    ///
    /// assert!(list.contains(&'a'));
    /// assert!(!list.contains(&'x'));
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        self.iter_ptr()
            .any(|p| self.0.node(p).data().is_some_and(|d| d == value))
    }

    /// ***O(n)*** Performs a forward search from the front and returns the position of the first node with value equal to the given `value`.
    ///
    /// Returns None if there is no element with the given value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::from_iter(['a', 'b', 'c', 'd']);
    ///
    /// let x = list.position_of_value(&'x');
    /// assert_eq!(x, None);
    ///
    /// let b = list.position_of_value(&'b'); // O(n)
    /// assert_eq!(b, Some(1));
    /// ```
    pub fn position_of_value(&self, value: &T) -> Option<usize> {
        self.iter_ptr().enumerate().find_map(|(i, p)| {
            self.0
                .node(p)
                .data()
                .is_some_and(|d| d == value)
                .then_some(i)
        })
    }
}

// doubly

impl<T, M, P> List<Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    T: PartialEq,
    P: PinnedVec<Node<Doubly<T>>>,
{
    /// ***O(n)*** Performs a forward search from the front and returns the index of the first node with value equal to the given `value`.
    ///
    /// Returns None if there is no element with the given value.
    ///
    /// Obtained `NodeIdx` can later be used for constant time access to the corresponding element.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::from_iter(['a', 'b', 'c', 'd']);
    ///
    /// let x = list.idx_of(&'x');
    /// assert!(x.is_none());
    ///
    /// let b = list.idx_of(&'b'); // O(n)
    /// assert!(b.is_some());
    ///
    /// let b = b.unwrap();
    ///
    /// let data_b = list.get(b); // O(1)
    /// assert_eq!(data_b, Some(&'b'));
    ///
    /// // O(1) to create the iterators from the index
    /// assert_eq!(&['b', 'c', 'd'], list.iter_from(b).copied().collect::<Vec<_>>().as_slice());
    /// assert_eq!(&['b', 'a'], list.iter_backward_from(b).copied().collect::<Vec<_>>().as_slice());
    ///
    /// list.insert_prev_to(b, 'X'); // O(1)
    /// list.insert_next_to(b, 'Y'); // O(1)
    /// assert!(list.eq_to_iter_vals(['a', 'X', 'b', 'Y', 'c', 'd']));
    ///
    /// let removed = list.remove(b);  // O(1)
    /// assert_eq!(removed, 'b');
    /// assert!(list.eq_to_iter_vals(['a', 'X', 'Y', 'c', 'd']));
    ///
    /// assert_eq!(list.get(b), None);
    /// assert_eq!(list.idx_err(b), Some(NodeIdxError::RemovedNode));
    /// ```
    pub fn idx_of(&self, value: &T) -> Option<DoublyIdx<T>> {
        self.iter_ptr()
            .find(|p| self.0.node(*p).data().is_some_and(|d| d == value))
            .map(|p| NodeIdx::new(self.memory_state(), p))
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
    /// let list: DoublyList<_> = ['a', 'b', 'c', 'd'].into_iter().collect();
    ///
    /// assert!(list.contains(&'a'));
    /// assert!(!list.contains(&'x'));
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        self.iter_ptr()
            .any(|p| self.0.node(p).data().is_some_and(|d| d == value))
    }

    /// ***O(n)*** Performs a forward search from the front and returns the position of the first node with value equal to the given `value`.
    ///
    /// Returns None if there is no element with the given value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::from_iter(['a', 'b', 'c', 'd']);
    ///
    /// let x = list.position_of_value(&'x');
    /// assert_eq!(x, None);
    ///
    /// let b = list.position_of_value(&'b'); // O(n)
    /// assert_eq!(b, Some(1));
    /// ```
    pub fn position_of_value(&self, value: &T) -> Option<usize> {
        self.iter_ptr().enumerate().find_map(|(i, p)| {
            self.0
                .node(p)
                .data()
                .is_some_and(|d| d == value)
                .then_some(i)
        })
    }

    /// ***O(n)*** Performs a backward search from the back and returns the index of the first node with value equal to the given `value`.
    ///
    /// Returns None if there is no element with the given value.
    ///
    /// Obtained `NodeIdx` can later be used for constant time access to the corresponding element.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::from_iter(['a', 'b', 'c', 'd']);
    ///
    /// let x = list.idx_of(&'x');
    /// assert!(x.is_none());
    ///
    /// let b = list.idx_of_from_back(&'b'); // O(n)
    /// assert!(b.is_some());
    ///
    /// let b = b.unwrap();
    ///
    /// let data_b = list.get(b); // O(1)
    /// assert_eq!(data_b, Some(&'b'));
    ///
    /// // O(1) to create the iterators from the index
    /// assert_eq!(&['b', 'c', 'd'], list.iter_from(b).copied().collect::<Vec<_>>().as_slice());
    /// assert_eq!(&['b', 'a'], list.iter_backward_from(b).copied().collect::<Vec<_>>().as_slice());
    ///
    /// list.insert_prev_to(b, 'X'); // O(1)
    /// list.insert_next_to(b, 'Y'); // O(1)
    /// assert!(list.eq_to_iter_vals(['a', 'X', 'b', 'Y', 'c', 'd']));
    ///
    /// let removed = list.remove(b);  // O(1)
    /// assert_eq!(removed, 'b');
    /// assert!(list.eq_to_iter_vals(['a', 'X', 'Y', 'c', 'd']));
    ///
    /// assert_eq!(list.get(b), None);
    /// assert_eq!(list.idx_err(b), Some(NodeIdxError::RemovedNode));
    /// ```
    pub fn idx_of_from_back(&self, value: &T) -> Option<DoublyIdx<T>> {
        self.iter_ptr()
            .rev()
            .find(|p| self.0.node(*p).data().is_some_and(|d| d == value))
            .map(|p| NodeIdx::new(self.memory_state(), p))
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
    /// let list: DoublyList<_> = ['a', 'b', 'c', 'd'].into_iter().collect();
    ///
    /// assert!(list.contains(&'a'));
    /// assert!(!list.contains(&'x'));
    /// ```
    pub fn contains_from_back(&self, value: &T) -> bool {
        self.iter_ptr()
            .rev()
            .any(|p| self.0.node(p).data().is_some_and(|d| d == value))
    }

    /// ***O(n)*** Performs a backward search from the back and returns the position of the first node with value equal to the given `value`.
    ///
    /// Returns None if there is no element with the given value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::from_iter(['a', 'b', 'c', 'd']);
    ///
    /// let x = list.position_of_from_back(&'x');
    /// assert_eq!(x, None);
    ///
    /// let b = list.position_of_from_back(&'b'); // O(n)
    /// assert_eq!(b, Some(2));
    /// ```
    pub fn position_of_from_back(&self, value: &T) -> Option<usize> {
        self.iter_ptr().rev().enumerate().find_map(|(i, p)| {
            self.0
                .node(p)
                .data()
                .is_some_and(|d| d == value)
                .then_some(i)
        })
    }
}
