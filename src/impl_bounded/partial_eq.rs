use crate::{linked_list::LinkedList, node::LinkedListNode};
use orx_imp_vec::prelude::PinnedVec;

impl<'a, T, P> LinkedList<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    /// Returns `true` if the `LinkedList` contains an element equal to the
    /// given value.
    ///
    /// Note that, as usual, this method compares value equality.
    ///
    /// This operation should compute linearly in *O*(*n*) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_doubling_growth(8);
    /// list.push_back(0);
    /// list.push_back(1);
    /// list.push_back(2);
    ///
    /// assert_eq!(list.contains(&0), true);
    /// assert_eq!(list.contains(&10), false);
    /// ```
    pub fn contains(&self, x: &T) -> bool {
        self.iter().any(|e| e == x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains() {
        let mut list = LinkedList::with_linear_growth(8);

        assert!(!list.contains(&'x'));

        list.push_back('y');
        assert!(!list.contains(&'x'));

        list.push_back('x');
        assert!(list.contains(&'x'));

        list.push_front('z');
        assert!(list.contains(&'x'));

        // z -> y -> x

        _ = list.pop_front();
        assert!(list.contains(&'x'));

        _ = list.pop_back();
        assert!(!list.contains(&'x'));
    }
}
