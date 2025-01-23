#![allow(unused_imports, dead_code)]
use crate::{
    type_aliases::{BACK_IDX, FRONT_IDX},
    variant::Doubly,
    DoublyEnds, DoublyIterable, List,
};
use core::fmt::Debug;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node, NodePtr};

type PtrAndNode<'a, T> = (NodePtr<Doubly<T>>, &'a Node<Doubly<T>>);

impl<T, M, P> List<Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    T: Debug + PartialEq + Eq,
    P: PinnedVec<Node<Doubly<T>>>,
{
    /// A debugging method that performs internal structural test on the list and panics if it is in an invalid state.
    ///
    /// # Panics
    ///
    /// Panics if the list is in an invalid state.
    #[cfg(feature = "validation")]
    #[allow(clippy::unwrap_used)]
    pub fn validate(&self) {
        let num_active_nodes = self.0.nodes().iter().filter(|x| x.is_active()).count();
        assert_eq!(num_active_nodes, self.len());
        assert_eq!(self.iter().count(), num_active_nodes);

        match num_active_nodes {
            0 => {
                assert!(self.front().is_none());
                assert!(self.back().is_none());
            }
            1 => {
                assert!(self.front().is_some());
                assert_eq!(self.front(), self.back());
                assert_eq!(self.0.ends().get(BACK_IDX), self.0.ends().get(FRONT_IDX));

                let front_ptr = self.0.ends().get(FRONT_IDX).unwrap();
                assert!(self.next(front_ptr).is_none());
                assert!(self.prev(front_ptr).is_none());

                let back_ptr = self.0.ends().get(BACK_IDX).unwrap();
                assert!(self.next(back_ptr).is_none());
                assert!(self.prev(back_ptr).is_none());
            }
            _ => {
                assert!(self.front().is_some());
                assert_ne!(self.0.ends().get(BACK_IDX), self.0.ends().get(FRONT_IDX));

                let mut fwd_pointers = alloc::vec![];
                let mut ptr = self.0.ends().get(FRONT_IDX).cloned().unwrap();
                fwd_pointers.push(ptr.clone());
                while let Some((next_ptr, next)) = self.next(&ptr) {
                    assert_eq!(next.prev().get(), Some(&ptr));
                    ptr = next_ptr;
                    fwd_pointers.push(ptr.clone());
                }
                assert_eq!(fwd_pointers.len(), num_active_nodes);

                let mut bwd_pointers = alloc::vec![];
                let mut ptr = self.0.ends().get(BACK_IDX).cloned().unwrap();
                bwd_pointers.push(ptr.clone());
                while let Some((prev_ptr, prev)) = self.prev(&ptr) {
                    assert_eq!(prev.next().get(), Some(&ptr));
                    ptr = prev_ptr;
                    bwd_pointers.push(ptr.clone());
                }

                bwd_pointers.reverse();
                assert_eq!(fwd_pointers, bwd_pointers);
            }
        }

        // data - fwd
        let mut iter = self.iter();

        assert_eq!(iter.next(), self.front());

        let mut maybe_ptr = self.0.ends().get(FRONT_IDX).cloned();
        for _ in 1..num_active_nodes {
            let ptr = maybe_ptr.clone().unwrap();
            maybe_ptr = self.next(&ptr).map(|x| x.0);

            let data = maybe_ptr
                .clone()
                .map(|p| unsafe { self.0.data_unchecked(&p) });
            assert_eq!(iter.next(), data);
        }
        assert!(iter.next().is_none());

        // data - bwd
        let mut iter = self.iter().rev();

        assert_eq!(iter.next(), self.back());

        let mut maybe_ptr = self.0.ends().get(BACK_IDX).cloned();
        for _ in 1..num_active_nodes {
            let ptr = maybe_ptr.clone().unwrap();
            maybe_ptr = self.prev(&ptr).map(|x| x.0);

            let data = maybe_ptr
                .clone()
                .map(|p| unsafe { self.0.data_unchecked(&p) });
            assert_eq!(iter.next(), data);
        }
        assert!(iter.next().is_none());
    }

    fn next(&self, ptr: &NodePtr<Doubly<T>>) -> Option<PtrAndNode<T>> {
        self.0.node(ptr).next().get().map(|p| {
            let next_node = self.0.node(p);
            (p.clone(), next_node)
        })
    }

    fn prev(&self, ptr: &NodePtr<Doubly<T>>) -> Option<PtrAndNode<T>> {
        self.0.node(ptr).prev().get().map(|p| {
            let prev_node = self.0.node(p);
            (p.clone(), prev_node)
        })
    }
}
