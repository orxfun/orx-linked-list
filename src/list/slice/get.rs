use super::list_slice::ListSlice;
use crate::variant::ListVariant;
use orx_selfref_col::{MemoryPolicy, Refs};

impl<V, M> ListSlice<'_, V, M>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
{
    /// Returns the number of elements in the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use orx_linked_list::*;
    ///
    /// let mut list = SinglyList::new();
    ///
    /// assert!(list.is_empty());
    ///
    /// list.push_front('a');
    /// assert!(!list.is_empty());
    ///
    /// _ = list.pop_front();
    /// assert!(list.is_empty());
    /// ```
    #[inline(always)]
    pub fn is_empty(&self) -> bool where {
        !self.ends.is_empty()
    }
}
