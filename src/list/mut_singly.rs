use super::List;
use crate::{SinglyIdx, iter::SinglyIterMut, variant::Singly};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node, NodeIdx, Refs};

impl<T, M, P> List<Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>>,
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
    /// let mut list = SinglyList::new();
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
        match self.0.ends().get() {
            Some(p) => Some(self.0.swap_data(p, new_front)),
            None => {
                self.push_front(new_front);
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
    pub fn push_front(&mut self, value: T) -> SinglyIdx<T> {
        let idx = self.0.push(value);

        if let Some(front) = self.0.ends().get() {
            self.0.node_mut(idx).next_mut().set_some(front);
        }

        self.0.ends_mut().set_some(idx);

        NodeIdx::new(self.0.memory_state(), idx)
    }

    /// ***O(1)*** Pops and returns the value at the `front` of the list; returns None if the list is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = SinglyList::new();
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
        self.0.ends().get().map(|front| {
            match self.0.node(front).next().get() {
                Some(new_front) => self.0.ends_mut().set_some(new_front),
                None => self.0.ends_mut().clear(),
            }
            self.0.close_and_reclaim(front)
        })
    }

    /// Returns a forward iterator of mutable references to elements of the list from front to back.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = SinglyList::new();
    ///
    /// list.push_front(2);
    /// list.push_front(1);
    /// list.push_front(0);
    /// assert!(list.eq_to_iter_vals([0, 1, 2]));
    ///
    /// for x in list.iter_mut() {
    ///     *x += 40;
    /// }
    ///
    /// assert!(list.eq_to_iter_vals([40, 41, 42]));
    /// ```
    pub fn iter_mut(&mut self) -> SinglyIterMut<'_, T, P> {
        SinglyIterMut::new_old(&mut self.0)
    }
}
