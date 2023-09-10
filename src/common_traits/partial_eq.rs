use crate::{node::LinkedListNode, LinkedList};
use orx_imp_vec::prelude::{FixedVec, Growth, PinnedVec, SplitVec};

impl<'a, T, P> PartialEq for LinkedList<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

// FixedVec
impl<'a, T, P> PartialEq<FixedVec<T>> for LinkedList<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &FixedVec<T>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}
impl<'a, T, P> PartialEq<LinkedList<'a, T, P>> for FixedVec<T>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &LinkedList<'a, T, P>) -> bool {
        other == self
    }
}

// SplitVec
impl<'a, T, P, G> PartialEq<SplitVec<T, G>> for LinkedList<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
    G: Growth,
{
    fn eq(&self, other: &SplitVec<T, G>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}
impl<'a, T, P, G> PartialEq<LinkedList<'a, T, P>> for SplitVec<T, G>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
    G: Growth,
{
    fn eq(&self, other: &LinkedList<'a, T, P>) -> bool {
        other == self
    }
}

// Vec
impl<'a, T, P> PartialEq<LinkedList<'a, T, P>> for Vec<T>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &LinkedList<'a, T, P>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}
impl<'a, T, P> PartialEq<Vec<T>> for LinkedList<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &Vec<T>) -> bool {
        other == self
    }
}

// [T]
impl<'a, T, P> PartialEq<[T]> for LinkedList<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &[T]) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}
impl<'a, T, P> PartialEq<LinkedList<'a, T, P>> for [T]
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &LinkedList<'a, T, P>) -> bool {
        other == self
    }
}

// [T;N]
impl<'a, T, P, const N: usize> PartialEq<[T; N]> for LinkedList<'a, T, P>
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &[T; N]) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}
impl<'a, T, P, const N: usize> PartialEq<LinkedList<'a, T, P>> for [T; N]
where
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
    T: PartialEq,
{
    fn eq(&self, other: &LinkedList<'a, T, P>) -> bool {
        other == self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const N: usize = 1000;

    fn get_list<'a>() -> (LinkedList<'a, usize>, Vec<usize>) {
        let mut list = LinkedList::new();
        let mut vec = vec![];
        for i in 0..N {
            list.push_back(i + 2);
            vec.push(i + 2);
        }
        (list, vec)
    }

    #[test]
    fn linked_list() {
        assert_eq!(get_list().0, get_list().0);
        assert_eq!(get_list().0, get_list().0.built());
        assert_eq!(get_list().0.built(), get_list().0);
        assert_eq!(get_list().0.built(), get_list().0.built());
    }

    #[test]
    fn fixed_vec() {
        fn fixed() -> FixedVec<usize> {
            get_list().1.into()
        }
        assert_eq!(get_list().0, fixed());
        assert_eq!(fixed(), get_list().0);

        assert_eq!(get_list().0.built(), fixed());
        assert_eq!(fixed(), get_list().0.built());
    }

    #[test]
    fn split_vec() {
        fn split() -> SplitVec<usize> {
            let mut split = SplitVec::with_doubling_growth(2);
            split.extend_from_slice(&get_list().1);
            split
        }
        assert_eq!(get_list().0, split());
        assert_eq!(split(), get_list().0);

        assert_eq!(get_list().0.built(), split());
        assert_eq!(split(), get_list().0.built());
    }

    #[test]
    fn vec() {
        assert_eq!(get_list().0, get_list().1);
        assert_eq!(get_list().1, get_list().0);

        assert_eq!(get_list().0.built(), get_list().1);
        assert_eq!(get_list().1, get_list().0.built());
    }

    #[test]
    fn slice() {
        assert_eq!(&get_list().0, get_list().1.as_slice());
        assert_eq!(get_list().1.as_slice(), &get_list().0);

        assert_eq!(&get_list().0.built(), get_list().1.as_slice());
        assert_eq!(get_list().1.as_slice(), &get_list().0.built());
    }

    #[test]
    fn const_slice() {
        fn get_slice() -> [usize; N] {
            let mut cs = [0usize; N];
            let vals = &get_list().1;
            for (i, x) in vals.iter().enumerate() {
                cs[i] = *x;
            }
            cs
        }
        assert_eq!(get_list().0, get_slice());
        assert_eq!(get_slice(), get_list().0);

        assert_eq!(get_list().0.built(), get_slice());
        assert_eq!(get_slice(), get_list().0.built());
    }
}
