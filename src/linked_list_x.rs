use crate::{linked_list::IS_SOME, node::LinkedListNode, LinkedList};
use orx_imp_vec::prelude::{PinnedVec, SplitVec};
use std::marker::PhantomData;

/// `LinkedListX` is a **structurally immutable** `LinkedList`.
/// In other words, it is a linked list which is done building:
///
/// * no insertions or removals are allowed from the list,
/// * the list cannot be cleared.
///
/// On the other hand, having a mut reference, the data of the elements
/// can still be mutated.
///
/// Note that it is possible to convert a `LinkedList` to a `LinkedListX`
/// using `built(self)` and vice versa by `continue_building(self)` methods.
/// Both methods are consuming and the conversions are cheap.
///
/// # Examples
///
/// Example below demonstrates how to switch between `LinkedList` and `LinkedListX`
/// to designate whether structural mutations are allowed or not.
///
/// ```rust
/// use orx_linked_list::prelude::*;
///
/// #[derive(Debug, PartialEq)]
/// struct Wizard {
///     name: String,
///     level: u32,
/// }
/// impl Wizard {
///     fn new<S: Into<String>>(name: S, level: u32) -> Self {
///         Self {
///             name: name.into(),
///             level,
///         }
///     }
///     fn cast_spell(&self) {}
/// }
///
/// // build the structure: both the structure and elements are mutable
/// let mut list: LinkedList<'_, Wizard> = LinkedList::new();
/// list.push_back(Wizard::new("Gandalf", 99));
/// list.push_front(Wizard::new("Sauron", 72));
/// list.push_back(Wizard::new("Palpatine", 71));
/// list.push_front(Wizard::new("Dumbledore", 86));
/// assert_eq!(
///     vec!["Dumbledore", "Sauron", "Gandalf", "Palpatine"],
///     list.iter().map(|w| &w.name).collect::<Vec<_>>()
/// );
///
/// // absolute immutable
/// let list: LinkedListX<'_, Wizard> = list.built();
/// for w in list.iter() {
///     w.cast_spell();
/// }
///
/// // back to structural mutations
/// let mut list: LinkedList<'_, Wizard> = list.continue_building();
/// let dumbledore = list.pop_front();
/// assert_eq!(dumbledore, Some(Wizard::new("Dumbledore", 86)));
/// list.push_back(Wizard::new("Merlin", 94));
/// assert_eq!(
///     vec!["Sauron", "Gandalf", "Palpatine", "Merlin"],
///     list.iter().map(|w| &w.name).collect::<Vec<_>>()
/// );
///
/// // stop structural mutations; however keep elements mutable
/// let mut list: LinkedListX<'_, Wizard> = list.built();
/// list.get_mut_at(1).unwrap().level += 1;
/// assert_eq!(list.get_at(1), Some(&Wizard::new("Gandalf", 100)));
///
/// // back to structural mutations
/// let mut list = list.continue_building();
/// let gandalf = list.remove_at(1);
/// assert_eq!(gandalf, Wizard::new("Gandalf", 100));
///
/// // freeze again
/// let list = list.built();
/// ```
///
/// # On structural immutablility
///
/// Together with the choice between immutable or `mut` variable, `LinkedList` and LinkedListX`
/// address the fact that im|mutability of collections is more than a boolean choice.
///
/// See the complete possibilities here:
///
/// | type              | element mutability | structural mutability | useful when/as |
/// | :---              |        :----:      |         :----:        | :---   |
/// |     `LinkedList`  | -                  | -                     | not really, see *Immutable `LinkedList` vs Immutable `LinkedListX`* |
/// | `mut LinkedList`  | +                  | +                     | while building the linked list |
/// |     `LinkedListX` | -                  | -                     | as an absolute immutable built linked list |
/// | `mut LinkedListX` | +                  | -                     | as a structurally immutable built linked list; while values of nodes can be mutated but relations cannot be |
///
///
/// ## Immutable `LinkedList` vs Immutable `LinkedListX`
/// As you may see an immutable `LinkedList` is not very useful.
///
/// ```rust
/// use orx_linked_list::prelude::*;
///
/// let list = LinkedList::<'_, char>::new();
/// ```
/// There is nothing to do with this `list`, it is and will be an empty list.
///
/// The situation is common. For instance, when we cannot convenitently `collect`,
/// we create a `mut std::vec::Vec`, build it up and move it
/// to an immutable variable to make sure that the built vector will not be mutated.
///
/// Linked list is a self referencing data structure which is much harder to `collect`.
/// In other words, building will probably require a `mut` reference.
/// Therefore, this approach seems to fit and can be used in the same manner as in the
/// example below.
///
/// ```rust
/// use orx_linked_list::prelude::*;
///
/// let mut list = LinkedList::new();
/// list.push_back('a');
/// let list: LinkedList<_> = list; // building is complete, no undesired mutation is allowed
/// ```
///
/// However, to be able to build the inter-element relations via thin references,
/// `LinkedList` makes use of `ImpVec`. `ImpVec` wraps a `PinnedVec` giving it the
/// ability to define such relations with cheap thin references. It achieves this
/// with one additional level of indirection which wraps the `PinnedVec` that
/// actually holds the data.
///
/// If the structural mutation is completed, we do not need and will not use
/// the ability to build inter-element references. Therefore, we can get rid of the
/// additional indirection and achieve the access performance of `PinnedVec`.
///
/// The transformation is convenient and cheap, and bidirectional.
///
/// Therefore, the usage below would be preferable than the example above:
///
/// ```rust
/// use orx_linked_list::prelude::*;
///
/// let mut list = LinkedList::new();
/// list.push_back('a');
/// let list: LinkedListX<_> = list.built(); // building is complete, no undesired mutation is allowed
/// ```
#[derive(Default)]
pub struct LinkedListX<'a, T, P = SplitVec<LinkedListNode<'a, T>>>
where
    T: 'a,
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
{
    pub(crate) vec: P,
    pub(crate) len: usize,
    pub(crate) marker: PhantomData<&'a T>,
}

impl<'a, T, P> LinkedListX<'a, T, P>
where
    T: 'a,
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
{
    /// Converts the `LinkedListX` back to a `LinkedList` allowing for **structural mutability**.
    ///
    /// See the documentation of [`LinkedListX`] for details.
    ///
    /// The `LinkedList` is created with `MemoryUtilization::default()`, which can be updated
    /// with `with_memory_utilization` method if needed.
    pub fn continue_building(self) -> LinkedList<'a, T, P> {
        LinkedList {
            vec: self.vec.into(),
            len: self.len,
            memory_utilization: Default::default(),
        }
    }

    /// Returns the length of the linked list.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::new();
    /// assert_eq!(0, list.len());
    ///
    /// list.push_back('a');
    /// list.push_front('b');
    /// assert_eq!(2, list.len());
    ///
    /// _ = list.pop_back();
    /// assert_eq!(1, list.len());
    ///
    /// let list = list.built();
    /// assert_eq!(1, list.len());
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }
    /// Returns true if the LinkedList is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::new();
    /// assert!(list.is_empty());
    ///
    /// list.push_front('a');
    /// list.push_front('b');
    /// assert!(!list.is_empty());
    ///
    /// let list = list.built();
    /// assert!(!list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Provides a reference to the back element, or None if the list is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_doubling_growth(4);
    /// assert_eq!(list.back(), None);
    ///
    /// list.push_back(42);
    /// assert_eq!(list.back(), Some(&42));
    ///
    /// list.push_front(1);
    /// assert_eq!(list.back(), Some(&42));
    ///
    /// list.push_back(7);
    /// assert_eq!(list.back(), Some(&7));
    ///
    /// let list = list.built();
    /// assert_eq!(list.back(), Some(&7));
    /// ```
    pub fn back(&self) -> Option<&T> {
        self.back_node().and_then(|node| node.data.as_ref())
    }
    /// Provides a mutable reference to the back element, or None if the list is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(16);
    /// assert_eq!(list.back(), None);
    ///
    /// list.push_back(42);
    /// assert_eq!(list.back(), Some(&42));
    ///
    /// let mut list = list.built();
    /// assert_eq!(list.back(), Some(&42));
    ///
    /// match list.back_mut() {
    ///     None => {},
    ///     Some(x) => *x = 7,
    /// }
    /// assert_eq!(list.back(), Some(&7));
    /// ```
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.node_ind(self.back_node())
            .and_then(|ind| self.vec.get_mut(ind).expect(IS_SOME).data.as_mut())
    }
    /// Provides a reference to the front element, or None if the list is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_doubling_growth(4);
    /// assert_eq!(list.front(), None);
    ///
    /// list.push_front(42);
    /// assert_eq!(list.front(), Some(&42));
    ///
    /// list.push_back(1);
    /// assert_eq!(list.front(), Some(&42));
    ///
    /// list.push_front(7);
    /// assert_eq!(list.front(), Some(&7));
    ///
    /// let list = list.built();
    /// assert_eq!(list.front(), Some(&7));
    /// ```
    pub fn front(&self) -> Option<&T> {
        self.front_node().and_then(|node| node.data.as_ref())
    }
    /// Provides a mutable reference to the front element, or None if the list is empty.
    ///
    /// This operation should compute in O(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(16);
    /// assert_eq!(list.front(), None);
    ///
    /// list.push_front(42);
    /// assert_eq!(list.front(), Some(&42));
    ///
    /// let mut list = list.built();
    /// assert_eq!(list.front(), Some(&42));
    ///
    /// match list.front_mut() {
    ///     None => {},
    ///     Some(x) => *x = 7,
    /// }
    /// assert_eq!(list.front(), Some(&7));
    /// ```
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.node_ind(self.front_node())
            .and_then(|ind| self.vec.get_mut(ind).expect(IS_SOME).data.as_mut())
    }

    /// Returns a reference to element at the `at` position starting from the `front`;
    /// None when `at` is out of bounds.
    ///
    /// This operation requires *O*(*n*) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(8);
    ///
    /// // build linked list: a <-> b <-> c
    /// list.push_back('b');
    /// list.push_front('a');
    /// list.push_back('c');
    ///
    /// let list = list.built();
    ///
    /// assert_eq!(Some(&'a'), list.get_at(0));
    /// assert_eq!(Some(&'b'), list.get_at(1));
    /// assert_eq!(Some(&'c'), list.get_at(2));
    /// assert_eq!(None, list.get_at(3));
    /// ```
    pub fn get_at(&self, at: usize) -> Option<&T> {
        if at < self.len {
            self.node_at(at).data.as_ref()
        } else {
            None
        }
    }
    /// Returns a mutable reference to element at the `at` position starting from the `front`;
    /// None when `at` is out of bounds.
    ///
    /// This operation requires *O*(*n*) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(8);
    ///
    /// // build linked list: a <-> b <-> c
    /// list.push_back('b');
    /// list.push_front('a');
    /// list.push_back('c');
    ///
    /// let mut list = list.built();
    ///
    /// *list.get_mut_at(0).unwrap() = 'x';
    /// *list.get_mut_at(1).unwrap() = 'y';
    /// *list.get_mut_at(2).unwrap() = 'z';
    /// assert_eq!(None, list.get_mut_at(3));
    ///
    /// assert_eq!(Some(&'x'), list.get_at(0));
    /// assert_eq!(Some(&'y'), list.get_at(1));
    /// assert_eq!(Some(&'z'), list.get_at(2));
    /// ```
    #[allow(clippy::unwrap_in_result)]
    pub fn get_mut_at(&mut self, at: usize) -> Option<&mut T> {
        if at < self.len {
            let node = self.node_at(at);
            let ind = self.node_ind(Some(node))?;
            self.vec.get_mut(ind).expect(IS_SOME).data.as_mut()
        } else {
            None
        }
    }

    // helpers
    /// Returns index of the referenced node:
    ///
    /// * might return None only if `node.is_none()`;
    /// * when `node.is_some()` it is expected to be a valid reference;
    /// hence, the method panics if not.
    ///
    /// # Safety
    ///
    /// Since this method, as well as the `LinkedListNode` struct are internal
    /// to this crate; it is never expected to receive an argument where the
    /// Some variant of the reference does not belong to the underlying imp
    /// vector.
    /// Therefore, `expect` call in the method body will never panic.
    pub(crate) fn node_ind(&self, node: Option<&'a LinkedListNode<'a, T>>) -> Option<usize> {
        node.map(|node_ref| self.vec.index_of(node_ref).expect(IS_SOME))
    }
    #[inline(always)]
    #[allow(clippy::unwrap_in_result)]
    pub(crate) fn back_node(&self) -> Option<&'a LinkedListNode<'a, T>> {
        self.vec.get(0).expect(IS_SOME).next
    }
    #[inline(always)]
    #[allow(clippy::unwrap_in_result)]
    pub(crate) fn front_node(&self) -> Option<&'a LinkedListNode<'a, T>> {
        self.vec.get(0).expect(IS_SOME).prev
    }
    fn node_at(&self, at: usize) -> &'a LinkedListNode<'a, T> {
        self.panic_if_out_of_bounds(at);
        let mut curr = self.vec.get(0).expect(IS_SOME).prev.expect(IS_SOME);
        for _ in 0..at {
            curr = curr.next.expect(IS_SOME);
        }
        curr
    }
    fn panic_if_out_of_bounds(&self, idx: usize) {
        assert!(
            idx < self.len,
            "Cannot remove at an index outside of the list bounds"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LinkedList, MemoryUtilization};

    #[test]
    fn len_is_empty() {
        let new = || LinkedList::with_doubling_growth(4);

        let list = new().built();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        let mut list = new();
        list.push_back(1);
        let list = list.built();
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);

        let mut list = new();
        list.push_back(1);
        list.push_front(2);
        let list = list.built();
        assert!(!list.is_empty());
        assert_eq!(list.len(), 2);

        let mut list = new();
        list.push_back(1);
        list.push_front(2);
        list.pop_back();
        let list = list.built();
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);

        let mut list = new();
        list.push_back(1);
        list.push_front(2);
        list.pop_back();
        list.pop_front();
        let list = list.built();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        let mut list = new();
        list.push_back(1);
        list.push_front(2);
        list.pop_back();
        list.pop_front();
        list.push_back(1);
        list.clear();
        let list = list.built();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn back_front() {
        let new = || LinkedList::with_linear_growth(16);

        let list = new().built();
        assert_eq!(None, list.back());
        assert_eq!(None, list.front());

        let mut list = new();
        list.push_back("hello");
        let list = list.built();
        assert_eq!(Some(&"hello"), list.back());
        assert_eq!(Some(&"hello"), list.front());

        let mut list = new();
        list.push_back("hello");
        list.push_front("world");
        let list = list.built();
        assert_eq!(Some(&"hello"), list.back());
        assert_eq!(Some(&"world"), list.front());

        let mut list = new();
        list.push_back("hello");
        list.push_front("world");
        list.push_back("hello");
        list.push_back("World");
        let list = list.built();
        assert_eq!(Some(&"World"), list.back());
        assert_eq!(Some(&"world"), list.front());

        let mut list = new();
        list.push_back("hello");
        list.push_front("world");
        list.push_back("hello");
        list.push_back("world");
        list.push_back("!");
        let list = list.built();
        assert_eq!(Some(&"!"), list.back());
    }

    #[test]
    fn back_front_mut() {
        let new = || LinkedList::<usize, _>::with_exponential_growth(8, 1.25);

        let mut list = new().built();
        assert_eq!(None, list.back_mut());
        assert_eq!(None, list.front_mut());

        // 10 - 20 - 30 - 40
        let mut list = new();
        list.push_back(20);
        list.push_back(30);
        list.push_front(10);
        list.push_back(40);
        let mut list = list.built();

        let back = list.back_mut();
        assert_eq!(Some(&mut 40), back);
        *back.expect("is-some") *= 10;
        assert_eq!(Some(&400), list.back());

        let front = list.front_mut();
        assert_eq!(Some(&mut 10), front);
        *front.expect("is-some") *= 10;
        assert_eq!(Some(&100), list.front());
    }

    #[test]
    fn get_at() {
        let new = || LinkedList::<usize, _>::with_doubling_growth(4);

        let mut list = new();
        for i in 0..1000 {
            list.push_back(i);
        }
        let mut list = list.built();

        for i in 0..list.len() {
            assert_eq!(Some(&i), list.get_at(i));
        }
        assert_eq!(None, list.get_at(1000));

        for i in 0..list.len() {
            *list.get_mut_at(i).expect(IS_SOME) *= 10;
        }
        assert_eq!(None, list.get_mut_at(1000));

        for i in 0..list.len() {
            assert_eq!(Some(&(i * 10)), list.get_at(i));
        }
        assert_eq!(None, list.get_at(1000));
    }

    #[test]
    fn built_and_continue_building() {
        let mut list = LinkedList::new().with_memory_utilization(MemoryUtilization::Lazy);
        list.push_back('c');
        list.push_front('b');
        list.push_back('d');
        list.push_back('e');
        list.push_front('a');
        list.pop_back();
        assert_eq!(vec!['a', 'b', 'c', 'd'], list.collect_vec());

        let list = list.built();
        assert_eq!(vec!['a', 'b', 'c', 'd'], list.collect_vec());

        let list = list.continue_building();
        assert_eq!(vec!['a', 'b', 'c', 'd'], list.collect_vec());
        assert_eq!(MemoryUtilization::default(), list.memory_utilization);
    }
}
