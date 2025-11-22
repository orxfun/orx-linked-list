use crate::variant::Singly;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{CoreCol, MemoryReclaimer, Node, NodePtr};

#[derive(Clone, Default)]
pub struct SinglyReclaimer;

impl<T> MemoryReclaimer<Singly<T>> for SinglyReclaimer {
    fn reclaim_nodes<P>(col: &mut CoreCol<Singly<T>, P>) -> bool
    where
        P: PinnedVec<Node<Singly<T>>>,
    {
        let mut nodes_moved = false;

        if let Some(mut occupied_ptr) = col.ends().get().cloned() {
            let mut prev = core::ptr::null();

            // SAFETY: lifetime of `forward` iterator is limited to this method
            // which is shorter than the lifetime of the `col`
            let forward = unsafe { col.nodes().iter_ptr() }.enumerate();

            for (v, vacant_ptr) in forward {
                if unsafe { &*vacant_ptr }.is_closed() {
                    loop {
                        let o = col.position_of_unchecked(&occupied_ptr);

                        let next = col.node(&occupied_ptr).next().get().cloned();

                        let swapped = o > v;
                        match swapped {
                            true => {
                                nodes_moved = true;
                                // SAFETY: we have a mutual &mut reference to the underlying collection
                                // which is guaranteed to be in the same memory state as occupied
                                swap(col, vacant_ptr, unsafe { occupied_ptr.ptr() }, prev);
                                prev = vacant_ptr;
                            }
                            // SAFETY: we have a mutual &mut reference to the underlying collection
                            // which is guaranteed to be in the same memory state as occupied
                            false => prev = unsafe { occupied_ptr.ptr() },
                        }

                        match next {
                            Some(next) => occupied_ptr = next,
                            None => return nodes_moved,
                        }

                        if swapped {
                            break;
                        }
                    }
                }
            }
        }

        nodes_moved
    }
}

fn swap<P, T>(
    col: &mut CoreCol<Singly<T>, P>,
    vacant: *const Node<Singly<T>>,
    occupied: *const Node<Singly<T>>,
    prev: *const Node<Singly<T>>,
) where
    P: PinnedVec<Node<Singly<T>>>,
{
    #[inline(always)]
    fn node_ptr<T>(p: *const Node<Singly<T>>) -> Option<NodePtr<Singly<T>>> {
        Some(NodePtr::new(p as *mut Node<Singly<T>>))
    }

    match prev.is_null() {
        false => {
            col.node_mut(&NodePtr::new(prev))
                .next_mut()
                .set(node_ptr(vacant));
        }
        true => col.ends_mut().set(node_ptr(vacant)), // must be the front
    }

    core::mem::swap(unsafe { &mut *(vacant as *mut Node<Singly<T>>) }, unsafe {
        &mut *(occupied as *mut Node<Singly<T>>)
    });
}
