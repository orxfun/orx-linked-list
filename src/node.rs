use orx_imp_vec::prelude::{SelfRefNext, SelfRefPrev};

/// A linked-list node holding the data together with previous & next references.
#[derive(Clone)]
pub struct Node<'a, T> {
    data: Option<T>,
    prev: Option<&'a Self>,
    next: Option<&'a Self>,
}

impl<'a, T> Node<'a, T> {
    /// Creates a new active node with some `data` and optional `prev` and `next` references.
    pub(crate) fn active(data: T, prev: Option<&'a Self>, next: Option<&'a Self>) -> Self {
        Self {
            data: Some(data),
            prev,
            next,
        }
    }

    /// Creates a clsoed node without data and empty prev & next references.
    pub(crate) fn closed() -> Self {
        Default::default()
    }

    /// Returns whether or not this node is closed.
    pub(crate) fn is_closed(&self) -> bool {
        self.data.is_none()
    }

    /// Returns a mutable reference to the data of the node if active; None otherwise.
    pub(crate) fn data_mut(&mut self) -> Option<&mut T> {
        self.data.as_mut()
    }

    /// Returns a reference to the data of the node if active.
    ///
    /// # Panics
    ///
    /// Panics if the node is a closed node with missing data.
    pub(crate) fn data_unchecked(&self) -> &T {
        self.data.as_ref().expect("is-some")
    }

    // test
    #[cfg(test)]
    /// Returns a reference to the data of the node if active; None otherwise.
    pub(crate) fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }
}

impl<'a, T> Default for Node<'a, T> {
    fn default() -> Self {
        Self {
            data: None,
            prev: None,
            next: None,
        }
    }
}

impl<'a, T> SelfRefNext<'a> for Node<'a, T> {
    #[inline(always)]
    fn next(&self) -> Option<&'a Self> {
        self.next
    }
    #[inline(always)]
    fn set_next(&mut self, next: Option<&'a Self>) {
        self.next = next;
    }
}

impl<'a, T> SelfRefPrev<'a> for Node<'a, T> {
    #[inline(always)]
    fn prev(&self) -> Option<&'a Self> {
        self.prev
    }
    #[inline(always)]
    fn set_prev(&mut self, prev: Option<&'a Self>) {
        self.prev = prev;
    }
}

impl<'a, T> From<Node<'a, T>> for Option<T> {
    fn from(value: Node<'a, T>) -> Self {
        value.data
    }
}

#[cfg(test)]
mod tests {
    use super::Node;
    use orx_imp_vec::prelude::{SelfRefNext, SelfRefPrev};

    #[test]
    fn new_node() {
        let other_node = Node {
            data: Some('a'),
            prev: None,
            next: None,
        };

        let node = Node::active('b', Some(&other_node), None);
        assert_eq!(Some('b'), node.data);
        assert!(node.next.is_none());
        assert_eq!(Some('a'), node.prev.and_then(|x| x.data));

        let node = Node::active('b', Some(&other_node), Some(&other_node));
        assert_eq!(Some('b'), node.data);
        assert_eq!(Some('a'), node.next.and_then(|x| x.data));
        assert_eq!(Some('a'), node.prev.and_then(|x| x.data));
    }
    #[test]
    fn closed_default() {
        fn assert_empty(node: Node<char>) {
            assert!(node.data.is_none());
            assert!(node.next.is_none());
            assert!(node.prev.is_none());
        }
        assert_empty(Node::closed());
        assert_empty(Node::default());
    }

    #[test]
    fn into_data() {
        let node: Node<'_, char> = Node::closed();
        let data: Option<char> = node.into();
        assert!(data.is_none());

        let node = Node::active('a', None, None);
        let data = node.into();
        assert_eq!(Some('a'), data);
    }

    #[test]
    fn is_closed() {
        let node: Node<'_, ()> = Node::closed();
        assert!(node.is_closed());

        let node: Node<'_, ()> = Node::default();
        assert!(node.is_closed());

        let node = Node::active('a', None, None);
        assert!(!node.is_closed());
    }

    #[test]
    fn data() {
        let mut node = Node::active('a', None, None);

        assert_eq!(Some(&'a'), node.data());
        assert_eq!(&'a', node.data_unchecked());

        if let Some(data) = node.data_mut() {
            *data = 'b';
        }
        assert_eq!(Some(&'b'), node.data());
        assert_eq!(&'b', node.data_unchecked());
    }
    #[test]
    #[should_panic]
    fn data_unchecked_for_closed_node() {
        let node: Node<'_, char> = Node::closed();
        let _data = node.data_unchecked();
    }

    #[test]
    fn set_next() {
        let mut node = Node::active('a', None, None);
        let other = Node::active('x', None, None);

        assert!(node.next().is_none());

        node.set_next(Some(&other));

        assert_eq!(Some(&'x'), node.next().and_then(|x| x.data()));

        node.set_next(None);
        assert!(node.next().is_none());
    }
    #[test]
    fn set_prev() {
        let mut node = Node::active('a', None, None);
        let other = Node::active('x', None, None);

        assert!(node.prev().is_none());

        node.set_prev(Some(&other));

        assert_eq!(Some(&'x'), node.prev().and_then(|x| x.data()));

        node.set_prev(None);
        assert!(node.prev().is_none());
    }
}
