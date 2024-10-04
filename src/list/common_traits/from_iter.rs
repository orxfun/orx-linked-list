use crate::{
    type_aliases::{BACK_IDX, FRONT_IDX},
    variant::Doubly,
    List, Singly,
};
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node, NodePtr, SelfRefCol};

// singly

impl<T, M> FromIterator<T> for List<Singly<T>, M>
where
    M: MemoryPolicy<Singly<T>>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        singly_from_iter(iter)
    }
}

pub(crate) fn singly_from_iter<T, M, P, I>(iter: I) -> List<Singly<T>, M, P>
where
    M: MemoryPolicy<Singly<T>>,
    P: PinnedVec<Node<Singly<T>>> + Default,
    I: IntoIterator<Item = T>,
{
    #[inline(always)]
    fn node_ptr<T>(p: *const Node<Singly<T>>) -> Option<NodePtr<Singly<T>>> {
        Some(NodePtr::new(p as *mut Node<Singly<T>>))
    }

    let mut col: SelfRefCol<Singly<T>, M, P> = SelfRefCol::from_iter(iter);

    // SAFETY: lifetime of the `forward` iterator is limited to this method
    // which is shorter than the lifetime of the `col`
    let mut forward = unsafe { col.nodes().iter_ptr() };

    if let Some(mut p) = forward.next() {
        col.ends_mut().set(node_ptr(p));
        let mut a = unsafe { &mut *(p as *mut Node<Singly<T>>) };
        for q in forward {
            a.next_mut().set(node_ptr(q));

            p = q;
            a = unsafe { &mut *(p as *mut Node<Singly<T>>) };
        }
    }

    List(col)
}

// doubly

impl<T, M> FromIterator<T> for List<Doubly<T>, M>
where
    M: MemoryPolicy<Doubly<T>>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        doubly_from_iter(iter)
    }
}

pub(crate) fn doubly_from_iter<T, M, P, I>(iter: I) -> List<Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>> + Default,
    I: IntoIterator<Item = T>,
{
    #[inline(always)]
    fn node_ptr<T>(p: *const Node<Doubly<T>>) -> Option<NodePtr<Doubly<T>>> {
        Some(NodePtr::new(p as *mut Node<Doubly<T>>))
    }

    let mut col: SelfRefCol<Doubly<T>, M, P> = SelfRefCol::from_iter(iter);

    // SAFETY: lifetime of the `forward` iterator is limited to this method
    // which is shorter than the lifetime of the `col`
    let mut forward = unsafe { col.nodes().iter_ptr() };

    if let Some(mut p) = forward.next() {
        col.ends_mut().set(FRONT_IDX, node_ptr(p));
        let mut a = unsafe { &mut *(p as *mut Node<Doubly<T>>) };
        for q in forward {
            a.next_mut().set(node_ptr(q));

            let b = unsafe { &mut *(q as *mut Node<Doubly<T>>) };
            b.prev_mut().set(node_ptr(p));

            p = q;
            a = b;
        }
        col.ends_mut().set(BACK_IDX, node_ptr(p));
    }

    List(col)
}
