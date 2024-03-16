use crate::variants::list_variant::ListVariant;
use orx_selfref_col::{Node, NodeRefSingle, NodeRefs};
use std::iter::FusedIterator;

pub struct IterForward<'iter, 'a, V, T>
where
    T: 'a,
    V: ListVariant<'a, T, Next = NodeRefSingle<'a, V, T>>,
{
    current: Option<&'iter Node<'a, V, T>>,
}

impl<'iter, 'a, V, T> IterForward<'iter, 'a, V, T>
where
    T: 'a,
    V: ListVariant<'a, T, Next = NodeRefSingle<'a, V, T>>,
{
    pub(crate) fn new(current: Option<&'iter Node<'a, V, T>>) -> Self {
        Self { current }
    }
}

impl<'iter, 'a, V, T> Iterator for IterForward<'iter, 'a, V, T>
where
    T: 'a,
    V: ListVariant<'a, T, Next = NodeRefSingle<'a, V, T>>,
{
    type Item = &'iter T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|x| {
            let data = unsafe { x.data().unwrap_unchecked() };
            self.current = *x.next().get();
            data
        })
    }
}

impl<'iter, 'a, V, T> FusedIterator for IterForward<'iter, 'a, V, T>
where
    T: 'a,
    V: ListVariant<'a, T, Prev = NodeRefSingle<'a, V, T>>,
{
}

impl<'iter, 'a, V, T> Clone for IterForward<'iter, 'a, V, T>
where
    T: 'a,
    V: ListVariant<'a, T, Next = NodeRefSingle<'a, V, T>>,
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
        variants::{doubly::Doubly, ends::ListEnds, list_variant::ListVariant, singly::Singly},
        DoublyLinkedList, SinglyLinkedList,
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
        let mut singly: List<Singly, _> = List::default();
        let mut doubly: List<Doubly, _> = List::default();

        for i in 0..100 {
            singly.push_front(99 - i);
            doubly.push_front(99 - i);
        }

        take_list_as_ref(&singly);
        take_list_as_ref(&doubly);

        let mut iter_singly = singly.iter();
        let mut iter_doubly = doubly.iter();
        for i in 0..100 {
            assert_eq!(Some(&i), iter_singly.next());
            assert_eq!(Some(&i), iter_doubly.next());
        }

        take_list_as_ref(&singly);
        take_list_as_ref(&doubly);

        take_list(singly);
        take_list(doubly);
    }

    #[test]
    fn iter() {
        let mut singly: List<Singly, _> = List::default();
        let mut doubly: List<Doubly, _> = List::default();

        for i in 0..100 {
            singly.push_front(99 - i);
            doubly.push_front(99 - i);
        }

        let mut iter_singly = singly.iter();
        let mut iter_doubly = doubly.iter();
        for i in 0..100 {
            assert_eq!(Some(&i), iter_singly.next());
            assert_eq!(Some(&i), iter_doubly.next());
        }

        assert!(iter_singly.next().is_none());
        assert!(iter_singly.next().is_none());

        assert!(iter_doubly.next().is_none());
        assert!(iter_doubly.next().is_none());
    }

    #[test]
    fn iter_from() {
        let mut singly: List<Singly, _> = List::default();
        let mut doubly: List<Doubly, _> = List::default();

        let mut indices_singly = vec![];
        let mut indices_doubly = vec![];
        for i in 0..100 {
            indices_singly.insert(0, singly.push_front(99 - i));
            indices_doubly.insert(0, doubly.push_front(99 - i));
        }

        for (start, idx) in indices_singly.iter().enumerate() {
            let mut iter_singly = singly.iter_forward_from(*idx).expect("is okay");
            for i in start..100 {
                assert_eq!(Some(&i), iter_singly.next());
            }
            assert!(iter_singly.next().is_none());
            assert!(iter_singly.next().is_none());
        }

        for (start, idx) in indices_doubly.iter().enumerate() {
            let mut iter_doubly = doubly.iter_forward_from(*idx).expect("is okay");
            for i in start..100 {
                assert_eq!(Some(&i), iter_doubly.next());
            }
            assert!(iter_doubly.next().is_none());
            assert!(iter_doubly.next().is_none());
        }
    }

    #[test]
    fn iter_from_wrong_index() {
        let other_singly = SinglyLinkedList::<i32>::new();
        let other_doubly = DoublyLinkedList::<i32>::new();
        let mut singly: List<Singly, _> = List::default();
        let mut doubly: List<Doubly, _> = List::default();

        let mut indices_singly = vec![];
        let mut indices_doubly = vec![];
        for i in 0..100 {
            indices_singly.insert(0, singly.push_front(99 - i));
            indices_doubly.insert(0, doubly.push_front(99 - i));
        }

        // wrong collection
        assert_eq!(
            other_singly.iter_forward_from(indices_singly[0]).err(),
            Some(NodeIndexError::WrongCollection)
        );
        assert_eq!(
            other_doubly.iter_forward_from(indices_doubly[0]).err(),
            Some(NodeIndexError::WrongCollection)
        );

        // remove back
        let removed = singly.remove_at(99);
        assert_eq!(removed, Some(99));
        let removed = doubly.remove_at(99);
        assert_eq!(removed, Some(99));

        assert_eq!(
            singly.iter_forward_from(indices_singly[99]).err(),
            Some(NodeIndexError::RemovedNode)
        );
        assert_eq!(
            doubly.iter_forward_from(indices_doubly[99]).err(),
            Some(NodeIndexError::RemovedNode)
        );

        // reorganized
        for i in 0..50 {
            singly.remove_at(98 - i);
            doubly.remove_at(98 - i);
        }
        assert_eq!(
            singly.iter_forward_from(indices_singly[0]).err(),
            Some(NodeIndexError::ReorganizedCollection)
        );
        assert_eq!(
            doubly.iter_forward_from(indices_doubly[0]).err(),
            Some(NodeIndexError::ReorganizedCollection)
        );
    }

    #[test]
    fn clone() {
        let mut singly: List<Singly, _> = List::default();
        let mut doubly: List<Doubly, _> = List::default();

        for i in 0..100 {
            singly.push_front(99 - i);
            doubly.push_front(99 - i);
        }

        let mut iter_singly = singly.iter();
        let mut iter_doubly = doubly.iter();
        for i in 0..50 {
            assert_eq!(Some(&i), iter_singly.next());
            assert_eq!(Some(&i), iter_doubly.next());
        }

        let mut iter_singly_2 = iter_singly.clone();
        let mut iter_doubly_2 = iter_doubly.clone();

        for i in 50..100 {
            assert_eq!(Some(&i), iter_singly_2.next());
            assert_eq!(Some(&i), iter_doubly_2.next());
        }

        assert!(iter_singly_2.next().is_none());
        assert!(iter_doubly_2.next().is_none());
    }
}
