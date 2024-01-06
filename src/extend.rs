use crate::LinkedList;

impl<'a, T> Extend<T> for LinkedList<'a, T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.push_back(x)
        }
    }
}

impl<'a, 'b, T: Copy> Extend<&'b T> for LinkedList<'a, T> {
    fn extend<I: IntoIterator<Item = &'b T>>(&mut self, iter: I) {
        for x in iter {
            self.push_back(*x)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::LinkedList;

    #[test]
    fn extend_non_copy() {
        let vec = vec!["a".to_string(), "b".to_string()];

        let mut list = LinkedList::new();
        list.extend(vec.clone());

        assert_eq!(&list, &vec);

        list.extend(vec.clone());
        let vec = vec![
            "a".to_string(),
            "b".to_string(),
            "a".to_string(),
            "b".to_string(),
        ];
        assert_eq!(&list, &vec);
    }

    #[test]
    fn extend_copy() {
        let vec = vec![0, 1];

        let mut list: LinkedList<usize> = LinkedList::new();
        list.extend(&vec);

        assert_eq!(&list, &vec);

        list.extend(3..5);
        assert_eq!(&list, &[0, 1, 3, 4]);
    }
}
