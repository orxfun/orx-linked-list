use super::List;
use crate::variant::ListVariant;
use orx_pinned_vec::PinnedVec;
use orx_selfref_col::{MemoryPolicy, MemoryReclaimer, MemoryState};

impl<V, M> List<V, M>
where
    V: ListVariant,
    M: MemoryPolicy<V>,
{
    /// Manually attempts to reclaim closed nodes.
    ///
    /// # Safety
    ///
    /// It is important to note that, memory reclaim operation might lead to reorganization of the nodes
    /// which invalidates the node indices obtained before the process.
    pub fn reclaim_closed_nodes(&mut self) -> (MemoryState, MemoryState) {
        let num_active_nodes = self.len();
        let old = self.0.memory_state();

        // let state_changed = SinglyReclaimer::reclaim(&mut self.0);
        let state_changed = <V::Reclaimer as MemoryReclaimer<V>>::reclaim_nodes(&mut self.0);
        self.0.nodes_mut().truncate(num_active_nodes);
        self.0.update_state(state_changed);

        (old, self.0.memory_state())
    }
}
