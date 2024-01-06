use crate::LinkedList;

impl<'a, T> FromIterator<T> for LinkedList<'a, T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for x in iter {
            list.push_back(x);
        }
        list
    }
}

#[cfg(test)]
mod tests {
    use crate::LinkedList;

    #[test]
    fn collect() {
        let list = LinkedList::<u32>::from_iter([0, 1, 2, 3, 4, 5].into_iter());
        assert_eq!(&list, &[0, 1, 2, 3, 4, 5]);

        let list: LinkedList<_> = (0..6).collect();
        assert_eq!(&list, &[0, 1, 2, 3, 4, 5]);

        let list: LinkedList<_> = (0..6).filter(|x| x % 2 == 0).collect();
        assert_eq!(&list, &[0, 2, 4]);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn collect_from_slice() {
        let list: LinkedList<_> = (0..6).collect();

        let (a, b) = list.split(3).unwrap();

        assert_eq!(a, &[0, 1, 2]);
        assert_eq!(b, &[3, 4, 5]);

        let list_a: LinkedList<_> = a.iter().copied().collect();
        let list_b: LinkedList<_> = b.iter().copied().collect();

        assert_eq!(&list_a, &[0, 1, 2]);
        assert_eq!(&list_b, &[3, 4, 5]);
    }
}
