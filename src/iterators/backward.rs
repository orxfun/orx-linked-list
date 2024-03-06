use std::iter::FusedIterator;

use crate::variants::list_variant::ListVariant;
use orx_selfref_col::{Node, NodeRefSingle, NodeRefs};

pub struct IterBackward<'iter, 'a, V, T>
where
    T: 'a,
    V: ListVariant<'a, T, Prev = NodeRefSingle<'a, V, T>>,
{
    current: Option<&'iter Node<'a, V, T>>,
}

impl<'iter, 'a, V, T> IterBackward<'iter, 'a, V, T>
where
    T: 'a,
    V: ListVariant<'a, T, Prev = NodeRefSingle<'a, V, T>>,
{
    pub(crate) fn new(current: Option<&'iter Node<'a, V, T>>) -> Self {
        Self { current }
    }
}

impl<'iter, 'a, V, T> Iterator for IterBackward<'iter, 'a, V, T>
where
    T: 'a,
    V: ListVariant<'a, T, Prev = NodeRefSingle<'a, V, T>>,
{
    type Item = &'iter T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|x| {
            let data = unsafe { x.data().unwrap_unchecked() };
            self.current = *x.prev().get();
            data
        })
    }
}

impl<'iter, 'a, V, T> FusedIterator for IterBackward<'iter, 'a, V, T>
where
    T: 'a,
    V: ListVariant<'a, T, Prev = NodeRefSingle<'a, V, T>>,
{
}

impl<'iter, 'a, V, T> Clone for IterBackward<'iter, 'a, V, T>
where
    T: 'a,
    V: ListVariant<'a, T, Prev = NodeRefSingle<'a, V, T>>,
{
    fn clone(&self) -> Self {
        Self {
            current: self.current,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        list::List,
        variants::{doubly::Doubly, ends::ListEnds, list_variant::ListVariant},
        DoublyLinkedList,
    };
    use orx_selfref_col::NodeIndexError;

    fn take_list<'a, V, T>(_: List<'a, V, T>)
    where
        V: ListVariant<'a, T>,
        V::Ends: ListEnds<'a, V, T>,
    {
    }

    fn take_list_as_ref<'a, V, T>(_: &List<'a, V, T>)
    where
        V: ListVariant<'a, T>,
        V::Ends: ListEnds<'a, V, T>,
    {
    }

    #[test]
    fn iter_lifetime() {
        let mut doubly: List<Doubly, _> = List::default();

        for i in 0..100 {
            doubly.push_front(99 - i);
        }

        take_list_as_ref(&doubly);

        let mut iter_doubly = doubly.iter_from_back();
        for i in 0..100 {
            assert_eq!(Some(&(99 - i)), iter_doubly.next());
        }

        take_list_as_ref(&doubly);

        take_list(doubly);
    }

    #[test]
    fn iter() {
        let mut doubly: List<Doubly, _> = List::default();

        for i in 0..100 {
            doubly.push_front(99 - i);
        }

        let mut iter_doubly = doubly.iter_from_back();
        for i in 0..100 {
            assert_eq!(Some(&(99 - i)), iter_doubly.next());
        }

        assert!(iter_doubly.next().is_none());
        assert!(iter_doubly.next().is_none());
    }

    #[test]
    fn iter_from() {
        let mut doubly: List<Doubly, _> = List::default();

        let mut indices = vec![];
        for i in 0..100 {
            indices.push(doubly.push_front(99 - i));
        }

        for (start, idx) in indices.iter().enumerate() {
            let mut iter_doubly = doubly.iter_backward_from(*idx).expect("is okay");
            for i in start..100 {
                assert_eq!(Some(&(99 - i)), iter_doubly.next());
            }

            assert!(iter_doubly.next().is_none());
            assert!(iter_doubly.next().is_none());
        }
    }

    #[test]
    fn iter_from_wrong_index() {
        let other_doubly = DoublyLinkedList::<i32>::new();
        let mut doubly: List<Doubly, _> = List::default();

        let mut indices_doubly = vec![];
        for i in 0..100 {
            indices_doubly.insert(0, doubly.push_front(99 - i));
        }

        // wrong collection
        assert_eq!(
            other_doubly.iter_backward_from(indices_doubly[0]).err(),
            Some(NodeIndexError::WrongCollection)
        );

        // remove back
        let removed = doubly.pop_back();
        assert_eq!(removed, Some(99));

        assert_eq!(
            doubly.iter_backward_from(indices_doubly[99]).err(),
            Some(NodeIndexError::RemovedNode)
        );

        // reorganized
        for i in 0..50 {
            doubly.remove_at(98 - i);
        }
        assert_eq!(
            doubly.iter_backward_from(indices_doubly[0]).err(),
            Some(NodeIndexError::ReorganizedCollection)
        );
    }

    #[test]
    fn clone() {
        let mut doubly: List<Doubly, _> = List::default();

        for i in 0..100 {
            doubly.push_front(99 - i);
        }

        let mut iter_doubly = doubly.iter_from_back();
        for i in 0..50 {
            assert_eq!(Some(&(99 - i)), iter_doubly.next());
        }

        let mut iter_doubly_2 = iter_doubly.clone();

        for i in 50..100 {
            assert_eq!(Some(&(99 - i)), iter_doubly_2.next());
        }

        assert!(iter_doubly_2.next().is_none());
    }
}
