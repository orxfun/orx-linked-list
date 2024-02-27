use crate::{Doubly, List, Singly};
use orx_selfref_col::SelfRefCol;

impl<'a, T> FromIterator<T> for List<'a, Singly, T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut col = SelfRefCol::from_iter(iter);

        col.move_mutate((), |x, _| {
            x.set_ends([x.first_node(), x.last_node()]);

            let len = x.len();
            if len >= 2 {
                let mut prev = x.first_node().expect("is-some");
                let mut current = x.get_node(1).expect("is-some");
                prev.set_next(&x, current);
                for i in 2..len {
                    prev = current;
                    current = x.get_node(i).expect("is-some");
                    prev.set_next(&x, current);
                }
            }
        });

        Self { col }
    }
}

impl<'a, T> FromIterator<T> for List<'a, Doubly, T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut col = SelfRefCol::from_iter(iter);

        col.move_mutate((), |x, _| {
            x.set_ends([x.first_node(), x.last_node()]);

            let len = x.len();
            if len >= 2 {
                let mut prev = x.first_node().expect("is-some");
                let mut current = x.get_node(1).expect("is-some");
                prev.set_next(&x, current);
                current.set_prev(&x, prev);
                for i in 2..len {
                    prev = current;
                    current = x.get_node(i).expect("is-some");
                    prev.set_next(&x, current);
                    current.set_prev(&x, prev);
                }
            }
        });

        Self { col }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    use self::list::tests::{assert_empty_list, validate_both};

    #[test]
    fn empty() {
        let vec: Vec<char> = vec![];
        let singly = SinglyLinkedList::from_iter(vec.clone());
        let doubly = DoublyLinkedList::from_iter(vec);

        assert_empty_list(&singly);
        assert_empty_list(&doubly);
        validate_both(&singly, &doubly);
    }

    #[test]
    fn single() {
        let vec: Vec<char> = vec!['a'];
        let singly = SinglyLinkedList::from_iter(vec.clone());
        let doubly = DoublyLinkedList::from_iter(vec.clone());

        assert_eq!(Some(&'a'), singly.front());
        assert_eq!(Some(&'a'), singly.back());
        assert_eq!(Some(&'a'), doubly.front());
        assert_eq!(Some(&'a'), doubly.back());
        assert_eq!(&vec, singly.iter().copied().collect::<Vec<_>>().as_slice());
        assert_eq!(&vec, doubly.iter().copied().collect::<Vec<_>>().as_slice());

        validate_both(&singly, &doubly);
    }

    #[test]
    fn double() {
        let vec: Vec<char> = vec!['a', 'b'];
        let singly = SinglyLinkedList::from_iter(vec.clone());
        let doubly = DoublyLinkedList::from_iter(vec.clone());

        assert_eq!(Some(&'a'), singly.front());
        assert_eq!(Some(&'b'), singly.back());
        assert_eq!(Some(&'a'), doubly.front());
        assert_eq!(Some(&'b'), doubly.back());
        assert_eq!(&vec, singly.iter().copied().collect::<Vec<_>>().as_slice());
        assert_eq!(&vec, doubly.iter().copied().collect::<Vec<_>>().as_slice());

        validate_both(&singly, &doubly);
    }

    #[test]
    fn multiple() {
        let vec: Vec<char> = vec!['a', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'b'];
        let singly = SinglyLinkedList::from_iter(vec.clone());
        let doubly = DoublyLinkedList::from_iter(vec.clone());

        assert_eq!(Some(&'a'), singly.front());
        assert_eq!(Some(&'b'), singly.back());
        assert_eq!(Some(&'a'), doubly.front());
        assert_eq!(Some(&'b'), doubly.back());
        assert_eq!(&vec, singly.iter().copied().collect::<Vec<_>>().as_slice());
        assert_eq!(&vec, doubly.iter().copied().collect::<Vec<_>>().as_slice());

        validate_both(&singly, &doubly);
    }
}
