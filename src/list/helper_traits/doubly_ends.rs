use super::{HasCol, HasColMut};
use crate::{
    type_aliases::{BACK_IDX, FRONT_IDX},
    Doubly, DoublyIdx,
};
use core::ops::RangeBounds;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node, NodeIdxError, NodePtr, Refs, Variant};

/// Lists and views with owned ends.
pub trait HasDoublyEnds<T, M, P>: HasCol<Doubly<T>, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    /// Returns a reference to the ends of the linked list.
    fn ends(&self) -> &<Doubly<T> as Variant>::Ends;

    fn range_start<'a, R: RangeBounds<&'a DoublyIdx<T>>>(
        &self,
        range: &R,
    ) -> Result<Option<NodePtr<Doubly<T>>>, NodeIdxError>
    where
        T: 'a,
    {
        use core::ops::Bound::*;

        let begin = match range.start_bound() {
            Excluded(x) => {
                let ptr = self.col().try_get_ptr(x)?;
                self.col().node(&ptr).next().get().cloned()
            }
            Included(x) => Some(self.col().try_get_ptr(x)?),
            Unbounded => self.col().ends().get(FRONT_IDX).cloned(),
        };

        Ok(begin)
    }

    fn range_end<'a, R: RangeBounds<&'a DoublyIdx<T>>>(
        &self,
        range: &R,
        front: &NodePtr<Doubly<T>>,
    ) -> Result<Option<NodePtr<Doubly<T>>>, NodeIdxError>
    where
        T: 'a,
    {
        use core::ops::Bound::*;

        let end = match range.end_bound() {
            Excluded(x) => {
                let ptr = self.col().try_get_ptr(x)?;
                match ptr == *front {
                    false => self.col().node(&ptr).prev().get().cloned(),
                    true => None,
                }
            }
            Included(x) => Some(self.col().try_get_ptr(x)?),
            Unbounded => self.ends().get(BACK_IDX).cloned(),
        };

        Ok(end)
    }

    fn slice_ends<'a, R>(&self, range: R) -> Result<<Doubly<T> as Variant>::Ends, NodeIdxError>
    where
        R: RangeBounds<&'a DoublyIdx<T>>,
        T: 'a,
    {
        Ok(match self.range_start(&range)? {
            Some(front) => {
                let back = self.range_end(&range, &front)?;
                match back {
                    Some(back) => {
                        let mut ends = <Doubly<T> as Variant>::Ends::empty();
                        ends.set_some(FRONT_IDX, front);
                        ends.set_some(BACK_IDX, back);
                        ends
                    }
                    _ => <Doubly<T> as Variant>::Ends::empty(),
                }
            }
            None => <Doubly<T> as Variant>::Ends::empty(),
        })
    }
}

/// Lists and views with owned mutable ends.
pub trait HasDoublyEndsMut<T, M, P>: HasColMut<Doubly<T>, M, P> + HasDoublyEnds<T, M, P>
where
    M: MemoryPolicy<Doubly<T>>,
    P: PinnedVec<Node<Doubly<T>>>,
{
    /// Returns a mutable reference to the ends of the linked list.
    fn ends_mut(&mut self) -> &mut <Doubly<T> as Variant>::Ends;

    // links
    #[inline(always)]
    fn is_linked(&self, prev: &NodePtr<Doubly<T>>, next: &NodePtr<Doubly<T>>) -> bool {
        self.col().node(prev).next().get() == Some(next)
            && self.col().node(next).prev().get() == Some(prev)
    }

    #[inline(always)]
    fn link(&mut self, prev: &NodePtr<Doubly<T>>, next: &NodePtr<Doubly<T>>) {
        self.col_mut()
            .node_mut(prev)
            .next_mut()
            .set_some(next.clone());
        self.col_mut()
            .node_mut(next)
            .prev_mut()
            .set_some(prev.clone());
    }

    #[inline(always)]
    fn unlink(&mut self, prev: &NodePtr<Doubly<T>>, next: &NodePtr<Doubly<T>>) {
        debug_assert!(self.is_linked(prev, next));

        self.col_mut().node_mut(prev).next_mut().set_none();
        self.col_mut().node_mut(next).prev_mut().set_none()
    }
}
