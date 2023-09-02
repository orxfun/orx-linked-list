use orx_imp_vec::prelude::SelfRefVecItem;

pub struct LinkedListNode<'a, T> {
    pub(crate) data: Option<T>,
    pub(crate) prev: Option<&'a LinkedListNode<'a, T>>,
    pub(crate) next: Option<&'a LinkedListNode<'a, T>>,
    pub(crate) ind: usize,
}
impl<'a, T> LinkedListNode<'a, T> {
    pub(crate) fn back_front_node() -> Self {
        Self::closed_node(0)
    }
    pub(crate) fn closed_node(ind: usize) -> Self {
        Self {
            data: None,
            prev: None,
            next: None,
            ind,
        }
    }
}

impl<'a, T> SelfRefVecItem<'a> for LinkedListNode<'a, T> {
    fn prev(&self) -> Option<&'a Self> {
        self.prev
    }
    fn next(&self) -> Option<&'a Self> {
        self.next
    }
    fn set_prev(&mut self, prev: Option<&'a Self>) {
        self.prev = prev;
    }
    fn set_next(&mut self, next: Option<&'a Self>) {
        self.next = next;
    }
}