use crate::variant::ListVariant;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, Node, NodeIdx, NodePtr, SelfRefCol};

/// Lists and views backed with a self-referential collection..
pub trait HasCol<V, M, P>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
    P: PinnedVec<Node<V>>,
{
    /// Returns a reference to the underlying self referential collection.
    fn col(&self) -> &SelfRefCol<V, M, P>;

    fn ptr_to_idx(&self, idx: NodePtr<V>) -> NodeIdx<V> {
        NodeIdx::new(self.col().memory_state(), idx)
    }
}

/// Lists and views backed with a self-referential collection..
pub trait HasColMut<V, M, P>: HasCol<V, M, P>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
    P: PinnedVec<Node<V>>,
{
    /// Returns a mutable reference to the underlying self referential collection.
    fn col_mut(&mut self) -> &mut SelfRefCol<V, M, P>;
}
