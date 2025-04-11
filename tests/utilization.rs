mod doubly;
mod singly;

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

// doubly

#[test]
fn doubly_lazy() {
    let mut rng = ChaCha8Rng::seed_from_u64(13);

    let mut list = doubly::new_doubly_lazy(&mut rng, 10, 20);

    let state = list.memory_state();

    let (old, new) = list.reclaim_closed_nodes();

    assert_eq!(state, old);
    assert_ne!(state, new);
    assert_eq!(list.node_utilization().num_closed_nodes, 0);
}

#[test]
fn doubly() {
    let mut rng = ChaCha8Rng::seed_from_u64(34);

    let mut list = doubly::new_doubly(&mut rng, 20, 20);

    list.pop_front();
    #[cfg(target_pointer_width = "64")]
    assert_eq!(list.node_utilization().num_closed_nodes, 6);
    #[cfg(target_pointer_width = "32")]
    assert_eq!(list.node_utilization().num_closed_nodes, 6);

    list.pop_front();
    #[cfg(target_pointer_width = "64")]
    assert_eq!(list.node_utilization().num_closed_nodes, 7);
    #[cfg(target_pointer_width = "32")]
    assert_eq!(list.node_utilization().num_closed_nodes, 7);

    list.pop_front();
    #[cfg(target_pointer_width = "64")]
    assert_eq!(list.node_utilization().num_closed_nodes, 0);
    #[cfg(target_pointer_width = "32")]
    assert_eq!(list.node_utilization().num_closed_nodes, 0);

    list.pop_back();

    list.clear();
    assert_eq!(list.node_utilization().num_closed_nodes, 0);
}

// singly

#[test]
fn singly_lazy() {
    let mut rng = ChaCha8Rng::seed_from_u64(5861);

    let mut list = singly::new_singly_lazy(&mut rng, 10, 20);

    let state = list.memory_state();

    let (old, new) = list.reclaim_closed_nodes();

    assert_eq!(state, old);
    assert_ne!(state, new);
    assert_eq!(list.node_utilization().num_closed_nodes, 0);
}

#[test]
fn singly() {
    let mut rng = ChaCha8Rng::seed_from_u64(34);

    let mut list = singly::new_singly(&mut rng, 20, 20);

    list.pop_front();

    list.pop_front();
    #[cfg(target_pointer_width = "64")]
    assert_eq!(list.node_utilization().num_closed_nodes, 1);
    #[cfg(target_pointer_width = "32")]
    assert_eq!(list.node_utilization().num_closed_nodes, 1);

    list.pop_front();
    assert_eq!(list.node_utilization().num_closed_nodes, 2);

    list.clear();
    assert_eq!(list.node_utilization().num_closed_nodes, 0);
}
