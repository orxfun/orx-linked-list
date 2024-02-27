use crate::{
    list::List,
    variants::{ends::ListEnds, list_variant::ListVariant},
};

impl<'a, V, T> Clone for List<'a, V, T>
where
    V: ListVariant<'a, T>,
    V::Ends: ListEnds<'a, V, T>,
    T: Clone,
    Self: FromIterator<T>,
{
    fn clone(&self) -> Self {
        Self::from_iter(self.iter().cloned())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn clone_empty() {
        let list = DoublyLinkedList::<char>::new();
        let clone = list.clone();
        assert!(clone.is_empty());

        let list = SinglyLinkedList::<char>::new();
        let clone = list.clone();
        assert!(clone.is_empty());
    }

    #[test]
    fn clone_single() {
        let mut list = DoublyLinkedList::<char>::new();
        list.push_back('a');
        let clone = list.clone();

        assert_eq!(1, clone.len());
        assert_eq!(Some(&'a'), clone.front());
        assert_eq!(Some(&'a'), clone.back());

        let mut list = SinglyLinkedList::<char>::new();
        list.push_front('a');
        let clone = list.clone();

        assert_eq!(1, clone.len());
        assert_eq!(Some(&'a'), clone.front());
        assert_eq!(Some(&'a'), clone.back());
    }

    #[test]
    fn clone_multi() {
        let mut list = DoublyLinkedList::<char>::new();
        list.push_back('a');
        list.push_back('b');
        list.push_back('c');
        let clone = list.clone();

        assert_eq!(3, clone.len());
        assert_eq!(Some(&'a'), clone.front());
        assert_eq!(Some(&'c'), clone.back());

        let mut list = SinglyLinkedList::<char>::new();
        list.push_front('c');
        list.push_front('b');
        list.push_front('a');
        let clone = list.clone();

        assert_eq!(3, clone.len());
        assert_eq!(Some(&'a'), clone.front());
        assert_eq!(Some(&'c'), clone.back());
    }
}
