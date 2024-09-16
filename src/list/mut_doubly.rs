use super::{helper_traits::HasDoublyEnds, List};
use crate::{
    type_aliases::{DoublyIdx, BACK_IDX, FRONT_IDX},
    variant::Doubly,
    ListSliceMut,
};
use core::ops::RangeBounds;
use orx_selfref_col::{MemoryPolicy, NodeIdx, Refs};

impl<T, M> List<Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
{
    /// ***O(1)*** Sets value of `front` of the list as `new_front` and:
    /// * returns value of the front element;
    /// * returns None if the list was empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
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
        match self.0.ends().get(FRONT_IDX) {
            Some(p) => Some(self.0.swap_data(&p, new_front)),
            None => {
                self.push_front(new_front);
                None
            }
        }
    }

    /// ***O(1)*** Sets value of `back` of the list as `new_back` and:
    /// * returns value of the back element;
    /// * returns None if the list was empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
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
        match self.0.ends().get(BACK_IDX) {
            Some(p) => Some(self.0.swap_data(&p, new_back)),
            None => {
                self.push_back(new_back);
                None
            }
        }
    }

    /// ***O(1)*** Pushes the `value` to the `front` of the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// list.push_front('a');
    /// list.push_front('b');
    ///
    /// assert_eq!(Some(&'b'), list.front());
    /// assert_eq!(Some(&'a'), list.back());
    ///
    /// let popped = list.pop_front();
    /// assert_eq!(Some('b'), popped);
    /// ```
    pub fn push_front(&mut self, value: T) -> DoublyIdx<T> {
        let idx = self.0.push(value);

        match self.0.ends().get(FRONT_IDX) {
            Some(front) => {
                self.0.node_mut(&front).prev_mut().set_some(&idx);
                self.0.node_mut(&idx).next_mut().set_some(&front);
                self.0.ends_mut().set_some(FRONT_IDX, &idx);
            }
            None => {
                self.0.ends_mut().set_some(FRONT_IDX, &idx);
                self.0.ends_mut().set_some(BACK_IDX, &idx);
            }
        }

        NodeIdx::new(self.0.memory_state(), &idx)
    }

    /// ***O(1)*** Pushes the `value` to the `back` of the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// list.push_back('a');
    /// list.push_back('b');
    ///
    /// assert_eq!(Some(&'b'), list.back());
    /// assert_eq!(Some(&'a'), list.front());
    ///
    /// let popped = list.pop_back();
    /// assert_eq!(Some('b'), popped);
    /// ```
    pub fn push_back(&mut self, value: T) -> DoublyIdx<T> {
        let idx = self.0.push(value);

        match self.0.ends().get(BACK_IDX) {
            Some(back) => {
                self.0.node_mut(&back).next_mut().set_some(&idx);
                self.0.node_mut(&idx).prev_mut().set_some(&back);
                self.0.ends_mut().set_some(BACK_IDX, &idx);
            }
            None => {
                self.0.ends_mut().set_some(FRONT_IDX, &idx);
                self.0.ends_mut().set_some(BACK_IDX, &idx);
            }
        }

        NodeIdx::new(self.0.memory_state(), &idx)
    }

    /// ***O(1)*** Pops and returns the value at the `front` of the list; returns None if the list is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
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
    pub fn pop_front(&mut self) -> Option<T> {
        self.0.ends().get(FRONT_IDX).map(|front| {
            match self.0.node(&front).next().get() {
                Some(new_front) => {
                    self.0.node_mut(&new_front).prev_mut().clear();
                    self.0.ends_mut().set_some(FRONT_IDX, &new_front);
                }
                None => self.0.ends_mut().clear(),
            }
            self.0.close_and_reclaim(&front)
        })
    }

    /// ***O(1)*** Pops and returns the value at the `back` of the list; returns None if the list is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// let popped = list.pop_back();
    /// assert!(popped.is_none());
    ///
    /// list.push_front('a');
    /// assert_eq!(Some(&'a'), list.front());
    ///
    /// let popped = list.pop_back();
    /// assert_eq!(Some('a'), popped);
    /// assert!(list.is_empty());
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        self.0.ends().get(BACK_IDX).map(|back| {
            match self.0.node(&back).prev().get() {
                Some(new_back) => {
                    self.0.node_mut(&new_back).next_mut().clear();
                    self.0.ends_mut().set_some(BACK_IDX, &new_back);
                }
                None => self.0.ends_mut().clear(),
            }
            self.0.close_and_reclaim(&back)
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
    /// let mut list = DoublyList::new();
    /// list.push_front('b');
    /// list.push_front('a');
    /// list.push_back('c');
    ///
    /// let other = DoublyList::from_iter(['d', 'e'].into_iter());
    ///
    /// list.append_front(other);
    /// assert!(list.eq_to_iter_vals(['d', 'e', 'a', 'b', 'c']));
    /// ```
    #[allow(clippy::missing_panics_doc)]
    pub fn append_front<M2: MemoryPolicy<Doubly<T>>>(&mut self, other: List<Doubly<T>, M2>) {
        let (col, other_state) = other.0.into_inner();
        let (nodes, ends, _len) = col.into_inner();

        self.0.append_nodes(nodes);

        let old_front_exists = !self.0.ends().is_empty();
        let new_front_exists = !ends.is_empty();

        match (old_front_exists, new_front_exists) {
            (_, false) => { /* no update when new is empty */ }
            (false, true) => {
                let new_front = ends.get(FRONT_IDX).expect("exists");
                self.0.ends_mut().set_some(FRONT_IDX, &new_front);
            }
            (true, true) => {
                let new_front = ends.get(FRONT_IDX).expect("exists");
                let new_back = ends.get(BACK_IDX).expect("exists");
                let old_front = self.0.ends().get(FRONT_IDX).expect("exists");

                self.0.node_mut(&old_front).prev_mut().set_some(&new_back);
                self.0.node_mut(&new_back).next_mut().set_some(&old_front);

                self.0.ends_mut().set_some(FRONT_IDX, &new_front);
            }
        }

        // update state if necessary
        if other_state != self.memory_state() {
            self.0.update_state(true);
            while self.memory_state() == other_state {
                self.0.update_state(true);
            }
        }
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
    /// let mut list = DoublyList::new();
    /// list.push_front('b');
    /// list.push_front('a');
    /// list.push_back('c');
    ///
    /// let other = DoublyList::from_iter(['d', 'e'].into_iter());
    ///
    /// list.append_back(other);
    /// assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd', 'e']));
    /// ```
    #[allow(clippy::missing_panics_doc)]
    pub fn append_back<M2: MemoryPolicy<Doubly<T>>>(&mut self, other: List<Doubly<T>, M2>) {
        let (col, other_state) = other.0.into_inner();
        let (nodes, ends, _len) = col.into_inner();

        self.0.append_nodes(nodes);

        let old_back_exists = !self.0.ends().is_empty();
        let new_back_exists = !ends.is_empty();

        match (old_back_exists, new_back_exists) {
            (_, false) => { /* no update when new is empty */ }
            (false, true) => {
                let new_back = ends.get(BACK_IDX).expect("exists");
                self.0.ends_mut().set_some(BACK_IDX, &new_back);
            }
            (true, true) => {
                let new_front = ends.get(FRONT_IDX).expect("exists");
                let new_back = ends.get(BACK_IDX).expect("exists");
                let old_back = self.0.ends().get(BACK_IDX).expect("exists");

                self.0.node_mut(&old_back).next_mut().set_some(&new_front);
                self.0.node_mut(&new_front).prev_mut().set_some(&old_back);

                self.0.ends_mut().set_some(BACK_IDX, &new_back);
            }
        }

        // update state if necessary
        if other_state != self.memory_state() {
            self.0.update_state(true);
            while self.memory_state() == other_state {
                self.0.update_state(true);
            }
        }
    }

    /// Creates and returns a slice of the list between the given `range` of indices.
    ///
    /// Note that a linked list slice itself also behaves like a linked list,
    /// reflecting the recursive nature of the data type.
    /// However, it does not own the data.
    /// It is rather a view, like a slice is a view to a vec.
    ///
    /// Note that slicing might be useful in various ways.
    /// For instance, we can keep indices of several critical elements of the list.
    /// We can then get all elements before, after or between any pair of these indices.
    /// Or we can combine the list with an indices vector, which provides the linked list
    /// a vec-like usage
    /// * with the disadvantage of using more memory, and
    /// * with the advantage of constant time insertions, removals or moves.
    ///
    /// # Panics
    ///
    /// Panics if any of indices of the range bounds is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = DoublyList::new();
    ///
    /// list.push_back(3);
    /// list.push_front(1);
    /// list.push_front(7);
    /// list.push_back(4);
    /// list.push_front(9);
    ///
    /// let expected_values = vec![9, 7, 1, 3, 4];
    ///
    /// assert!(list.eq_to_iter_refs(&expected_values));
    /// assert!(list.slice(..).eq_to_iter_refs(&expected_values));
    ///
    /// let idx: Vec<_> = list.indices().collect();
    ///
    /// let slice = list.slice(&idx[1]..=&idx[3]);
    /// assert_eq!(slice.front(), Some(&7));
    /// assert_eq!(slice.back(), Some(&3));
    /// assert!(slice.eq_to_iter_vals([7, 1, 3]));
    ///
    /// let sum: usize = slice.iter().sum();
    /// assert_eq!(sum, 11);
    /// ```
    ///
    /// Note that the linked list and its slices are directed.
    /// In other words, it does not by default have a cyclic behavior.
    /// Therefore, if the end of the `range` is before the beginning,
    /// the slice will stop at the `back` of the list.
    /// See the following example for clarification.
    ///
    /// Currently, cyclic or ring behavior can be achieved by `ring_iter` method.
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list: DoublyList<_> = (0..10).collect();
    /// let idx: Vec<_> = list.indices().collect();
    ///
    /// // a..b where b comes later, hence, we get the slice a..b
    /// let slice = list.slice_mut(&idx[1]..&idx[4]);
    /// assert!(slice.eq_to_iter_vals([1, 2, 3]));
    ///
    /// // a..b where b comes earlier, then, we get the slice a..back
    /// let slice = list.slice_mut(&idx[4]..&idx[1]);
    /// assert!(slice.eq_to_iter_vals([4, 5, 6, 7, 8, 9]));
    /// ```
    pub fn slice_mut<'a, R>(&mut self, range: R) -> ListSliceMut<Doubly<T>, M>
    where
        R: RangeBounds<&'a DoublyIdx<T>>,
        T: 'a,
    {
        let ends = self.slice_ends(range).expect("invalid indices in range");
        ListSliceMut { list: self, ends }
    }
}
