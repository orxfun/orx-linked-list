use crate::{linked_list_slice::LinkedListSlice, linked_list_view::LinkedListView, LinkedList};
use std::ops::Deref;

// view
impl<'a, T: PartialEq> PartialEq for LinkedListView<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        match (self.len(), other.len()) {
            (0, 0) => true,
            (a, b) if a != b => false,
            _ => {
                std::ptr::eq(
                    self.front_node().expect("issome"),
                    other.front_node().expect("issome"),
                ) && std::ptr::eq(
                    self.back_node().expect("issome"),
                    other.back_node().expect("issome"),
                ) || self.iter().zip(other.iter()).all(|(x, y)| x == y)
            }
        }
    }
}

impl<'s, 'a, T: PartialEq> PartialEq<LinkedListSlice<'s, 'a, T>> for LinkedListView<'a, T> {
    fn eq(&self, other: &LinkedListSlice<'s, 'a, T>) -> bool {
        let other = other.deref();
        self.eq(other.deref())
    }
}
impl<'a, T: PartialEq> PartialEq<LinkedList<'a, T>> for LinkedListView<'a, T> {
    fn eq(&self, other: &LinkedList<'a, T>) -> bool {
        let other = other.deref();
        self.eq(other.deref())
    }
}

impl<'a, T: PartialEq> PartialEq<[T]> for LinkedListView<'a, T> {
    fn eq(&self, other: &[T]) -> bool {
        other.eq(self)
    }
}

// asref(slice)
impl<'a, T: PartialEq, S: AsRef<[T]>> PartialEq<S> for LinkedListView<'a, T> {
    fn eq(&self, other: &S) -> bool {
        let other = other.as_ref();
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

impl<'s, 'a, T: PartialEq, S: AsRef<[T]>> PartialEq<S> for LinkedListSlice<'s, 'a, T> {
    fn eq(&self, other: &S) -> bool {
        self.deref().eq(other)
    }
}

impl<'a, T: PartialEq, S: AsRef<[T]>> PartialEq<S> for LinkedList<'a, T> {
    fn eq(&self, other: &S) -> bool {
        self.deref().eq(other)
    }
}

// slice
impl<'a, T: PartialEq> PartialEq<LinkedListView<'a, T>> for [T] {
    fn eq(&self, other: &LinkedListView<'a, T>) -> bool {
        let slice = self;
        other.len() == slice.len() && other.iter().zip(slice.iter()).all(|(x, y)| x == y)
    }
}
impl<'s, 'a, T: PartialEq> PartialEq<LinkedListSlice<'s, 'a, T>> for [T] {
    fn eq(&self, other: &LinkedListSlice<'s, 'a, T>) -> bool {
        self.eq(other.deref())
    }
}
impl<'a, T: PartialEq> PartialEq<LinkedList<'a, T>> for [T] {
    fn eq(&self, other: &LinkedList<'a, T>) -> bool {
        self.eq(other.deref())
    }
}

// listslice
impl<'s, 'a, T: PartialEq> PartialEq for LinkedListSlice<'s, 'a, T> {
    fn eq(&self, other: &LinkedListSlice<'s, 'a, T>) -> bool {
        self.deref().eq(other.deref())
    }
}
impl<'s, 'a, T: PartialEq> PartialEq<LinkedListView<'a, T>> for LinkedListSlice<'s, 'a, T> {
    fn eq(&self, other: &LinkedListView<'a, T>) -> bool {
        other.eq(self.deref())
    }
}
impl<'s, 'a, T: PartialEq> PartialEq<LinkedList<'a, T>> for LinkedListSlice<'s, 'a, T> {
    fn eq(&self, other: &LinkedList<'a, T>) -> bool {
        other.deref().eq(self.deref())
    }
}

// list
impl<'a, T: PartialEq> PartialEq for LinkedList<'a, T> {
    fn eq(&self, other: &LinkedList<'a, T>) -> bool {
        self.deref().eq(other.deref())
    }
}
impl<'a, T: PartialEq> PartialEq<LinkedListView<'a, T>> for LinkedList<'a, T> {
    fn eq(&self, other: &LinkedListView<'a, T>) -> bool {
        other.eq(self.deref())
    }
}
impl<'s, 'a, T: PartialEq> PartialEq<LinkedListSlice<'s, 'a, T>> for LinkedList<'a, T> {
    fn eq(&self, other: &LinkedListSlice<'s, 'a, T>) -> bool {
        other.deref().eq(self.deref())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use crate::LinkedList;

    #[test]
    fn eq_other() {
        let mut list1 = LinkedList::new();

        assert_eq!(list1, LinkedList::new());

        list1.push_back('a');
        list1.push_front('b');
        list1.push_front('c');
        list1.push_back('d');

        assert_eq!(&list1, &['c', 'b', 'a', 'd']);

        assert_eq!(list1, list1);
        assert_eq!(list1, list1.as_slice());
        assert_eq!(list1.as_slice(), list1);
        assert_eq!(list1.as_slice(), list1.as_slice());

        let (slice, _) = list1.split(4).unwrap();
        assert_eq!(slice, list1);
        assert_eq!(list1, slice);

        let (_, slice) = list1.split(0).unwrap();
        assert_eq!(slice, list1);
        assert_eq!(list1, slice);

        let (a, b) = list1.split(1).unwrap();
        assert_ne!(a, list1);
        assert_ne!(b, list1);
        assert_ne!(list1, a);
        assert_ne!(list1, b);

        let mut list2 = LinkedList::new();
        list2.push_back('a');
        list2.push_front('b');
        list2.push_front('c');
        assert_ne!(list1, list2);

        list2.push_back('d');
        assert_eq!(list1, list2);
    }

    #[test]
    fn eq_asref() {
        let mut list = LinkedList::new();

        list.push_back('a');
        list.push_front('b');
        list.push_front('c');
        list.push_back('d');

        assert_eq!(list, &['c', 'b', 'a', 'd']);
        assert_eq!(list.as_slice(), &['c', 'b', 'a', 'd']);

        let (a, b) = list.split(1).expect("within bounds");
        assert_eq!(a, &['c']);
        assert_eq!(b, &['b', 'a', 'd']);
    }
}
