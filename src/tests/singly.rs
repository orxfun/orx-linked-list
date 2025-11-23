#![allow(unused_imports, dead_code)]
use crate::{List, SinglyEnds, SinglyIterable, variant::Singly};
use core::fmt::Debug;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node, NodePtr};

type PtrAndNode<'a, T> = (NodePtr<Singly<T>>, &'a Node<Singly<T>>);

impl<T, M, P> List<Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    T: Debug + PartialEq + Eq,
    P: PinnedVec<Node<Singly<T>>>,
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
            }
            1 => {
                assert!(self.front().is_some());

                let front_ptr = self.0.ends().get().unwrap();
                assert!(self.next(front_ptr).is_none());
            }
            _ => {
                assert!(self.front().is_some());

                let mut fwd_pointers = alloc::vec![];
                let mut ptr = self.0.ends().get().cloned().unwrap();
                fwd_pointers.push(ptr.clone());
                while let Some((next_ptr, _)) = self.next(&ptr) {
                    ptr = next_ptr;
                    fwd_pointers.push(ptr.clone());
                }
                assert_eq!(fwd_pointers.len(), num_active_nodes);
            }
        }

        // data - fwd
        let mut iter = self.iter();

        assert_eq!(iter.next(), self.front());

        let mut maybe_ptr = self.0.ends().get().cloned();
        for _ in 1..num_active_nodes {
            let ptr = maybe_ptr.clone().unwrap();
            maybe_ptr = self.next(&ptr).map(|x| x.0);

            let data = maybe_ptr
                .clone()
                .map(|p| unsafe { self.0.data_unchecked(&p) });
            assert_eq!(iter.next(), data);
        }
        assert!(iter.next().is_none());
    }

    fn next(&self, ptr: NodePtr<Singly<T>>) -> Option<PtrAndNode<'_, T>> {
        self.0.node(ptr).next().get().map(|p| {
            let next_node = self.0.node(p);
            (p, next_node)
        })
    }
}
