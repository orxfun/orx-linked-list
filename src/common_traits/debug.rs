use crate::{
    list::List,
    variants::{doubly::Doubly, singly::Singly},
};
use std::fmt::Debug;

impl<'a, T> Debug for List<'a, Singly, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SinglyLinkedList")
            .field("len", &self.len())
            .field("front", &self.front())
            .field("forward", &self.iter().collect::<Vec<_>>())
            .finish()
    }
}

impl<'a, T> Debug for List<'a, Doubly, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DoublyLinkedList")
            .field("len", &self.len())
            .field("front", &self.front())
            .field("back", &self.back())
            .field("forward", &self.iter().collect::<Vec<_>>())
            .field("backward", &self.iter_from_back().collect::<Vec<_>>())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_singly_empty() {
        let list: List<Singly, usize> = List::new();

        assert_eq!(
            format!("{:?}", &list),
            "SinglyLinkedList { len: 0, front: None, forward: [] }"
        );
    }

    #[test]
    fn debug_singly_single() {
        let mut list: List<Singly, _> = List::new();
        list.push_front(42);

        assert_eq!(
            format!("{:?}", &list),
            "SinglyLinkedList { len: 1, front: Some(42), forward: [42] }"
        );
    }

    #[test]
    fn debug_singly_multi() {
        let mut list: List<Singly, _> = List::new();
        list.push_front(42);
        list.push_front(1);
        list.push_front(7);

        assert_eq!(
            format!("{:?}", &list),
            "SinglyLinkedList { len: 3, front: Some(7), forward: [7, 1, 42] }"
        );
    }

    #[test]
    fn debug_doubly_empty() {
        let list: List<Doubly, usize> = List::new();

        assert_eq!(
            format!("{:?}", &list),
            "DoublyLinkedList { len: 0, front: None, back: None, forward: [], backward: [] }"
        );
    }

    #[test]
    fn debug_doubly_single() {
        let mut list: List<Doubly, _> = List::new();
        list.push_front(42);

        assert_eq!(
            format!("{:?}", &list),
            "DoublyLinkedList { len: 1, front: Some(42), back: Some(42), forward: [42], backward: [42] }"
        );
    }

    #[test]
    fn debug_doubly_multi() {
        let mut list: List<Doubly, _> = List::new();
        list.push_front(42);
        list.push_front(1);
        list.push_front(7);

        assert_eq!(
            format!("{:?}", &list),
            "DoublyLinkedList { len: 3, front: Some(7), back: Some(42), forward: [7, 1, 42], backward: [42, 1, 7] }"
        );
    }
}
