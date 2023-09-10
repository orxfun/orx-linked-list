use crate::LinkedList;

impl<'a, T> Default for LinkedList<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}
