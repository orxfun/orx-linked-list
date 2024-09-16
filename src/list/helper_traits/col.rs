use crate::{type_aliases::PinVec, variant::ListVariant};
use orx_selfref_col::{MemoryPolicy, NodeIdx, NodePtr, SelfRefCol};

/// Lists and views backed with a self-referential collection..
pub trait HasCol<V, M>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
{
    /// Returns a reference to the underlying self referential collection.
    fn col(&self) -> &SelfRefCol<V, M, PinVec<V>>;

    fn ptr_to_idx(&self, idx: &NodePtr<V>) -> NodeIdx<V> {
        NodeIdx::new(self.col().memory_state(), idx)
    }
}

/// Lists and views backed with a self-referential collection..
pub trait HasColMut<V, M>: HasCol<V, M>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
{
    /// Returns a mutable reference to the underlying self referential collection.
    fn col_mut(&mut self) -> &mut SelfRefCol<V, M, PinVec<V>>;
}
