use crate::{linked_list::IS_SOME, node::LinkedListNode, LinkedList, LinkedListX};
use orx_imp_vec::{prelude::PinnedVec, ImpVec};
use std::fmt::Debug;

impl<'a, T, P> Clone for LinkedList<'a, T, P>
where
    T: 'a + Clone + Debug,
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
{
    fn clone(&self) -> Self {
        let mut imp: ImpVec<_, _> = unsafe { self.vec.unsafe_clone() }.into();
        for i in 0..imp.len() {
            if i == 0 || imp[i].data.is_some() {
                if let Some(prev) = self.vec[i].prev {
                    dbg!("prev", i, self.vec.index_of(prev));
                    imp.set_prev(i, self.vec.index_of(prev));
                }
                if let Some(next) = self.vec[i].next {
                    dbg!("next", i, self.vec.index_of(next));
                    imp.set_next(i, self.vec.index_of(next));
                }
            }
        }

        Self {
            vec: imp,
            len: self.len,
            memory_utilization: self.memory_utilization,
        }
    }
}
impl<'a, T, P> Clone for LinkedListX<'a, T, P>
where
    T: 'a + Clone,
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
{
    fn clone(&self) -> Self {
        let mut imp: ImpVec<_, _> = unsafe { self.vec.unsafe_clone() }.into();
        for i in 0..imp.len() {
            if i == 0 || imp[i].data.is_some() {
                if let Some(prev) = self.vec.get(i).expect(IS_SOME).prev {
                    imp.set_prev(i, self.vec.index_of(prev));
                }
                if let Some(next) = self.vec.get(i).expect(IS_SOME).next {
                    imp.set_next(i, self.vec.index_of(next));
                }
            }
        }

        Self {
            vec: imp.into_pinned(),
            len: self.len,
            marker: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone() {
        let list = || {
            let mut list = LinkedList::with_linear_growth(3);
            for i in 0..1000 {
                list.push_back(i);
                list.push_front(1000 + i);
                _ = list.pop_back();
            }
            list
        };

        let expected = list().collect_vec();

        let original = list();
        let clone = original.clone();
        drop(original);
        assert_eq!(expected, clone.collect_vec());

        let original = list().built();
        let clone = original.clone();
        drop(original);
        assert_eq!(expected, clone.collect_vec());

        let original = list().built().continue_building();
        let clone = original.clone();
        drop(original);
        assert_eq!(expected, clone.collect_vec());
    }

    #[test]
    fn validate_references() {
        let mut original = LinkedList::new();
        original.push_back('a');
        original.push_back('b');
        original.push_back('c');
        assert_eq!(vec!['a', 'b', 'c'], original.collect_vec());

        let clone = original.clone();
        assert_eq!(vec!['a', 'b', 'c'], clone.collect_vec());

        let originalx = original.built();
        let clonex = originalx.clone();
        assert_eq!(vec!['a', 'b', 'c'], clonex.collect_vec());

        // mutate original
        let mut original = originalx.continue_building();
        original.remove_at(1);
        original.memory_reclaim();
        original.push_back('d');
        original.push_front('e');
        original.pop_back();
        original.pop_front();
        original.pop_back();
        original.memory_reclaim();

        assert_eq!(vec!['a', 'b', 'c'], clone.collect_vec());
        assert_eq!(vec!['a', 'b', 'c'], clonex.collect_vec());
    }
}
