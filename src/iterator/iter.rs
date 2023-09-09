use crate::{linked_list::IS_SOME, node::LinkedListNode, prelude::LinkedList, LinkedListX};
use orx_imp_vec::prelude::PinnedVec;
use std::iter::FusedIterator;

impl<'a, T, P> LinkedList<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: 'a,
{
    /// Provides a forward iterator;
    /// which starts from the front-most element to the back.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(4);
    ///
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_front(0);
    /// list.push_back(3);
    ///
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter<'b>(&self) -> IterFromFront<'b, T>
    where
        'a: 'b,
    {
        IterFromFront {
            curr: self.vec[0].prev,
            len: self.len,
        }
    }
    /// Provides a backward iterator;
    /// which starts from the back-most element to the front.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(4);
    ///
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_front(0);
    /// list.push_back(3);
    ///
    /// let mut iter = list.iter_from_back();
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_from_back<'b>(&self) -> IterFromBack<'b, T>
    where
        'a: 'b,
    {
        IterFromBack {
            curr: self.vec[0].next,
            len: self.len,
        }
    }
}
impl<'a, T, P> LinkedListX<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: 'a,
{
    /// Provides a forward iterator;
    /// which starts from the front-most element to the back.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(4);
    ///
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_front(0);
    /// list.push_back(3);
    ///
    /// let list = list.built();
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter<'b>(&self) -> IterFromFront<'b, T>
    where
        'a: 'b,
    {
        IterFromFront {
            curr: self.vec.get(0).expect(IS_SOME).prev,
            len: self.len,
        }
    }
    /// Provides a backward iterator;
    /// which starts from the back-most element to the front.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_linked_list::prelude::*;
    ///
    /// let mut list = LinkedList::with_linear_growth(4);
    ///
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_front(0);
    /// list.push_back(3);
    ///
    /// let list = list.built();
    /// let mut iter = list.iter_from_back();
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_from_back<'b>(&self) -> IterFromBack<'b, T>
    where
        'a: 'b,
    {
        IterFromBack {
            curr: self.vec.get(0).expect(IS_SOME).next,
            len: self.len,
        }
    }
}

/// An iterator over the elements of a `LinkedList`
/// which starts from the `front` node and proceeds to the `back` node.
///
/// This struct is created by `LinkedList::iter()` method.
pub struct IterFromFront<'b, T> {
    curr: Option<&'b LinkedListNode<'b, T>>,
    len: usize,
}
impl<'b, T> Iterator for IterFromFront<'b, T> {
    type Item = &'b T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr_node) = self.curr {
            self.curr = curr_node.next;
            self.len -= 1;
            curr_node.data.as_ref()
        } else {
            None
        }
    }
}
impl<T> FusedIterator for IterFromFront<'_, T> {}
impl<T> ExactSizeIterator for IterFromFront<'_, T> {
    fn len(&self) -> usize {
        self.len
    }
}

// rev
/// An iterator over the elements of a `LinkedList`
/// which starts from the `back` node and proceeds to the `front` node.
///
/// This struct is created by `LinkedList::iter_from_back()` method.
pub struct IterFromBack<'b, T> {
    curr: Option<&'b LinkedListNode<'b, T>>,
    len: usize,
}
impl<'b, T> Iterator for IterFromBack<'b, T> {
    type Item = &'b T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr_node) = self.curr {
            self.curr = curr_node.prev;
            self.len -= 1;
            curr_node.data.as_ref()
        } else {
            None
        }
    }
}
impl<T> FusedIterator for IterFromBack<'_, T> {}
impl<T> ExactSizeIterator for IterFromBack<'_, T> {
    fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum TestWithType {
        WithLinkedList,
        WithLinkedListX,
    }

    #[test]
    fn next() {
        fn test(test_with_type: TestWithType) {
            let mut list = LinkedList::with_linear_growth(64);

            list.push_back(2);
            list.push_back(3);
            list.push_front(1);
            list.push_front(0);
            list.push_back(4);

            assert_eq!(vec![0, 1, 2, 3, 4], list.collect_vec());

            let mut iter = match test_with_type {
                TestWithType::WithLinkedList => list.iter(),
                TestWithType::WithLinkedListX => list.built().iter(),
            };

            assert_eq!(5, iter.len());

            assert_eq!(Some(&0), iter.next());
            assert_eq!(4, iter.len());

            assert_eq!(Some(&1), iter.next());
            assert_eq!(3, iter.len());

            assert_eq!(Some(&2), iter.next());
            assert_eq!(2, iter.len());

            assert_eq!(Some(&3), iter.next());
            assert_eq!(1, iter.len());

            assert_eq!(Some(&4), iter.next());
            assert_eq!(0, iter.len());

            assert_eq!(None, iter.next());
            assert_eq!(0, iter.len());

            assert_eq!(None, iter.next());
            assert_eq!(0, iter.len());
        }
        test(TestWithType::WithLinkedList);
        test(TestWithType::WithLinkedListX);
    }

    #[test]
    fn next_rev() {
        fn test(test_with_type: TestWithType) {
            let mut list = LinkedList::with_linear_growth(64);

            list.push_back(2);
            list.push_back(3);
            list.push_front(1);
            list.push_front(0);
            list.push_back(4);

            assert_eq!(vec![0, 1, 2, 3, 4], list.collect_vec());

            let mut iter = match test_with_type {
                TestWithType::WithLinkedList => list.iter_from_back(),
                TestWithType::WithLinkedListX => list.built().iter_from_back(),
            };

            assert_eq!(5, iter.len());

            assert_eq!(Some(&4), iter.next());
            assert_eq!(4, iter.len());

            assert_eq!(Some(&3), iter.next());
            assert_eq!(3, iter.len());

            assert_eq!(Some(&2), iter.next());
            assert_eq!(2, iter.len());

            assert_eq!(Some(&1), iter.next());
            assert_eq!(1, iter.len());

            assert_eq!(Some(&0), iter.next());
            assert_eq!(0, iter.len());

            assert_eq!(None, iter.next());
            assert_eq!(0, iter.len());

            assert_eq!(None, iter.next());
            assert_eq!(0, iter.len());
        }
        test(TestWithType::WithLinkedList);
        test(TestWithType::WithLinkedListX);
    }
}
