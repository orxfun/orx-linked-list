use crate::node::Node;
use orx_imp_vec::prelude::{SelfRefNext, SelfRefPrev};
use std::marker::PhantomData;

/// Linked list iteration direction.
pub trait IterDirection<'a, T> {
    /// Returns the next element of the given `node` wirth respect to the iteration direction.
    fn next_of(node: &'a Node<'a, T>) -> Option<&'a Node<'a, T>>;
}

/// Forward iteration from front to the back.
pub struct IterFromFront;
impl<'a, T> IterDirection<'a, T> for IterFromFront {
    #[inline(always)]
    fn next_of(node: &'a Node<'a, T>) -> Option<&'a Node<'a, T>> {
        node.next()
    }
}

/// Backward iteration from back to the front.
pub struct IterFromBack;
impl<'a, T> IterDirection<'a, T> for IterFromBack {
    #[inline(always)]
    fn next_of(node: &'a Node<'a, T>) -> Option<&'a Node<'a, T>> {
        node.prev()
    }
}

pub(crate) struct IterNodes<'a, T, Direction>
where
    Direction: IterDirection<'a, T>,
{
    remaining: usize,
    current: Option<&'a Node<'a, T>>,
    phantom: PhantomData<Direction>,
}
impl<'a, T, Direction> IterNodes<'a, T, Direction>
where
    Direction: IterDirection<'a, T>,
{
    pub(crate) fn new(len: usize, current: Option<&'a Node<'a, T>>) -> Self {
        Self {
            remaining: len,
            current,
            phantom: Default::default(),
        }
    }
}
impl<'a, T, Direction> Iterator for IterNodes<'a, T, Direction>
where
    Direction: IterDirection<'a, T>,
{
    type Item = &'a Node<'a, T>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            self.current.map(|x| {
                self.current = Direction::next_of(x);
                self.remaining -= 1;
                x
            })
        } else {
            None
        }
    }
}

/// Linked list iterator with the given `Direction`.
pub struct Iter<'a, T, Direction>(IterNodes<'a, T, Direction>)
where
    Direction: IterDirection<'a, T>;

impl<'a, T, Direction> From<IterNodes<'a, T, Direction>> for Iter<'a, T, Direction>
where
    Direction: IterDirection<'a, T>,
{
    fn from(value: IterNodes<'a, T, Direction>) -> Self {
        Self(value)
    }
}

impl<'a, T, Direction> Iterator for Iter<'a, T, Direction>
where
    Direction: IterDirection<'a, T>,
{
    type Item = &'a T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| x.data_unchecked())
    }
}
