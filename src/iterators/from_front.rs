use crate::variants::list_variant::ListVariant;
use orx_selfref_col::{Node, NodeRefSingle, NodeRefs};

pub struct IterFromFront<'iter, 'a, V, T>
where
    T: 'a,
    V: ListVariant<'a, T, Next = NodeRefSingle<'a, V, T>>,
{
    current: Option<&'iter Node<'a, V, T>>,
    len: usize,
}

impl<'iter, 'a, V, T> IterFromFront<'iter, 'a, V, T>
where
    T: 'a,
    V: ListVariant<'a, T, Next = NodeRefSingle<'a, V, T>>,
{
    pub(crate) fn new(len: usize, current: Option<&'iter Node<'a, V, T>>) -> Self {
        Self { current, len }
    }
}

impl<'iter, 'a, V, T> Iterator for IterFromFront<'iter, 'a, V, T>
where
    T: 'a,
    V: ListVariant<'a, T, Next = NodeRefSingle<'a, V, T>>,
{
    type Item = &'iter T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.len -= 1;
            let current = unsafe { self.current.unwrap_unchecked() };
            let data = unsafe { current.data().unwrap_unchecked() };
            self.current = *current.next().get();
            Some(data)
        } else {
            None
        }
    }
}

impl<'iter, 'a, V, T> ExactSizeIterator for IterFromFront<'iter, 'a, V, T>
where
    T: 'a,
    V: ListVariant<'a, T, Next = NodeRefSingle<'a, V, T>>,
{
    fn len(&self) -> usize {
        self.len
    }
}

impl<'iter, 'a, V, T> Clone for IterFromFront<'iter, 'a, V, T>
where
    T: 'a,
    V: ListVariant<'a, T, Next = NodeRefSingle<'a, V, T>>,
{
    fn clone(&self) -> Self {
        Self {
            current: self.current,
            len: self.len,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        list::List,
        variants::{doubly::Doubly, ends::ListEnds, list_variant::ListVariant, singly::Singly},
    };

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
    fn len() {
        let mut singly: List<Singly, _> = List::default();
        let mut doubly: List<Doubly, _> = List::default();

        for i in 0..100 {
            singly.push_front(99 - i);
            doubly.push_front(99 - i);
        }

        let mut iter_singly = singly.iter();
        let mut iter_doubly = doubly.iter();
        for i in 0..100 {
            assert_eq!(100 - i, iter_singly.len());
            assert_eq!(100 - i, iter_doubly.len());

            iter_singly.next();
            iter_doubly.next();
        }

        assert_eq!(0, iter_singly.len());
        assert_eq!(0, iter_doubly.len());
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
