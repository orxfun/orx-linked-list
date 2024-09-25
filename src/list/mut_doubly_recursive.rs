use super::List;
use crate::{
    type_aliases::{BACK_IDX, FRONT_IDX},
    variant::Doubly,
};
use orx_selfref_col::{MemoryPolicy, Node, Refs};
use orx_split_vec::{Recursive, SplitVec};

impl<T, M> List<Doubly<T>, M, SplitVec<Node<Doubly<T>>, Recursive>>
where
    M: MemoryPolicy<Doubly<T>>,
{
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
}
