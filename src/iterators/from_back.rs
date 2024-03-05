use crate::variants::doubly::Doubly;
use orx_selfref_col::{Node, NodeRefs};

pub struct IterFromBack<'iter, 'a, T: 'a> {
    current: Option<&'iter Node<'a, Doubly, T>>,
    len: usize,
}

impl<'iter, 'a, T> IterFromBack<'iter, 'a, T> {
    pub(crate) fn new(len: usize, current: Option<&'iter Node<'a, Doubly, T>>) -> Self {
        Self { current, len }
    }
}

impl<'iter, 'a, T> Iterator for IterFromBack<'iter, 'a, T> {
    type Item = &'iter T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.len -= 1;
            let current = unsafe { self.current.unwrap_unchecked() };
            let data = unsafe { current.data().unwrap_unchecked() };
            self.current = *current.prev().get();
            Some(data)
        } else {
            None
        }
    }
}

impl<'iter, 'a, T> ExactSizeIterator for IterFromBack<'iter, 'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'iter, 'a, T> Clone for IterFromBack<'iter, 'a, T> {
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
        variants::{doubly::Doubly, ends::ListEnds, list_variant::ListVariant},
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
    fn len() {
        let mut doubly: List<Doubly, _> = List::default();

        for i in 0..100 {
            doubly.push_front(99 - i);
        }

        let mut iter_doubly = doubly.iter_from_back();
        for i in 0..100 {
            assert_eq!(100 - i, iter_doubly.len());

            iter_doubly.next();
        }

        assert_eq!(0, iter_doubly.len());
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