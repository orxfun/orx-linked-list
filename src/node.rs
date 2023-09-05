use orx_imp_vec::prelude::SelfRefVecItem;

pub struct LinkedListNode<'a, T> {
    pub(crate) data: Option<T>,
    pub(crate) prev: Option<&'a LinkedListNode<'a, T>>,
    pub(crate) next: Option<&'a LinkedListNode<'a, T>>,
}
impl<'a, T> LinkedListNode<'a, T> {
    pub(crate) fn back_front_node() -> Self {
        Self::closed_node()
    }
    pub(crate) fn closed_node() -> Self {
        Self {
            data: None,
            prev: None,
            next: None,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ctors() {
        let node = LinkedListNode::<String>::back_front_node();
        assert!(node.prev.is_none());
        assert!(node.next.is_none());
        assert!(node.data.is_none());
    }

    #[test]
    fn prev_next() {
        let mut a = LinkedListNode {
            data: Some('a'),
            prev: None,
            next: None,
        };
        assert!(a.prev().is_none());
        assert!(a.next().is_none());

        let b = LinkedListNode {
            data: Some('b'),
            prev: None,
            next: None,
        };
        let c = LinkedListNode {
            data: Some('c'),
            prev: Some(&b),
            next: None,
        };

        a.set_next(Some(&b));
        a.set_prev(Some(&c));

        assert_eq!(Some('b'), a.next().and_then(|x| x.data));
        assert_eq!(Some('c'), a.prev().and_then(|x| x.data));

        a.set_next(None);
        a.set_prev(None);
        assert!(a.prev().is_none());
        assert!(a.next().is_none());
    }
}
