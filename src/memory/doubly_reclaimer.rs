use crate::{
    type_aliases::{BACK_IDX, FRONT_IDX},
    variant::Doubly,
};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{CoreCol, MemoryReclaimer, Node, NodePtr};

#[derive(Clone, Default)]
pub struct DoublyReclaimer;

impl DoublyReclaimer {
    pub fn reclaim<T, P>(col: &mut CoreCol<Doubly<T>, P>) -> bool
    where
        P: PinnedVec<Node<Doubly<T>>>,
    {
        let mut any_swapped = false;

        // SAFETY: lifetimes of `forward` and `backward` iterators are limited to this method
        // which is shorter than the lifetime of the `col`
        let forward = unsafe { col.nodes().iter_ptr() };
        let mut backward = unsafe { col.nodes().iter_ptr_rev() };
        let mut o = col.nodes().len();

        for (v, vacant_ptr) in forward.enumerate() {
            if v >= o {
                break;
            }

            if unsafe { &*vacant_ptr }.is_closed() {
                while o > v {
                    o -= 1;
                    let occupied_ptr = backward.next().expect("cannot be consumed before forward");

                    if unsafe { &*occupied_ptr }.is_active() {
                        any_swapped = true;
                        swap(col, vacant_ptr, occupied_ptr);
                        break;
                    }
                }
            }
        }

        any_swapped
    }
}

impl<T> MemoryReclaimer<Doubly<T>> for DoublyReclaimer {
    fn reclaim_nodes<P>(col: &mut CoreCol<Doubly<T>, P>) -> bool
    where
        P: PinnedVec<Node<Doubly<T>>>,
    {
        Self::reclaim(col)
    }
}

fn swap<P, T>(
    col: &mut CoreCol<Doubly<T>, P>,
    vacant: *const Node<Doubly<T>>,
    occupied: *const Node<Doubly<T>>,
) where
    P: PinnedVec<Node<Doubly<T>>>,
{
    #[inline(always)]
    fn node_ptr<T>(p: *const Node<Doubly<T>>) -> Option<NodePtr<Doubly<T>>> {
        Some(NodePtr::new(p as *mut Node<Doubly<T>>))
    }

    if let Some(prev) = (unsafe { &*occupied }).prev().get() {
        col.node_mut(prev).next_mut().set(node_ptr(vacant));
    }

    if let Some(next) = (unsafe { &*occupied }).next().get() {
        col.node_mut(next).prev_mut().set(node_ptr(vacant));
    }

    core::mem::swap(unsafe { &mut *(vacant as *mut Node<Doubly<T>>) }, unsafe {
        &mut *(occupied as *mut Node<Doubly<T>>)
    });

    if occupied == col.ends().get(FRONT_IDX).expect("nonempty list").ptr() {
        col.ends_mut().set(FRONT_IDX, node_ptr(vacant));
    }

    if occupied == col.ends().get(BACK_IDX).expect("nonempty list").ptr() {
        col.ends_mut().set(BACK_IDX, node_ptr(vacant));
    }
}
