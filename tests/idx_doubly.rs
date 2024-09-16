mod doubly;

use orx_linked_list::*;

#[test]
fn idx_get_lazy() {
    let mut list = DoublyListLazy::new();

    let idx0 = list.push_back(0);
    let _ = list.push_back(1);
    let _ = list.push_back(2);
    assert!(list.eq_to_iter_refs(&[0, 1, 2]));

    assert_eq!(list.try_get(&idx0), Ok(&0));

    *list.try_get_mut(&idx0).unwrap() = 42;
    assert_eq!(list.try_get(&idx0), Ok(&42));

    _ = list.pop_front();
    assert!(list.eq_to_iter_refs(&[1, 2]));
    assert_eq!(list.try_get(&idx0), Err(NodeIdxError::RemovedNode));
    assert_eq!(list.try_get_mut(&idx0), Err(NodeIdxError::RemovedNode));

    let idx3 = list.push_back(3);
    assert!(list.eq_to_iter_refs(&[1, 2, 3]));
    assert_eq!(list.try_get(&idx0), Err(NodeIdxError::RemovedNode));
    assert_eq!(list.try_get(&idx3), Ok(&3));

    // since we use DoublyList with 'lazy-reclaim' variant, unless we call 'reclaim_closed_nodes'
    // * state will never change
    // * indices will always be valid (Active or RemovedNode)
    list.reclaim_closed_nodes();

    assert_eq!(
        list.try_get(&idx0),
        Err(NodeIdxError::ReorganizedCollection)
    );
    assert_eq!(list.try_get(&idx3), Err(NodeIdxError::OutOfBounds));
}

#[test]
fn idx_get() {
    let mut list = DoublyList::new();

    let idx0 = list.push_back(0);
    let _ = list.push_back(1);
    let _ = list.push_back(2);
    assert!(list.eq_to_iter_refs(&[0, 1, 2]));

    assert_eq!(list.try_get(&idx0), Ok(&0));

    *list.try_get_mut(&idx0).unwrap() = 42;
    assert_eq!(list.try_get(&idx0), Ok(&42));

    _ = list.pop_front();
    assert!(list.eq_to_iter_refs(&[1, 2]));
    assert_eq!(
        list.try_get(&idx0),
        Err(NodeIdxError::ReorganizedCollection)
    );
    assert_eq!(
        list.try_get_mut(&idx0),
        Err(NodeIdxError::ReorganizedCollection)
    );

    list.clear();

    let _ = list.push_back(0);
    let idx1 = list.push_back(1);
    let _ = list.push_back(2);
    let _ = list.push_back(3);
    let idx4 = list.push_back(4);
    assert!(list.eq_to_iter_refs(&[0, 1, 2, 3, 4]));

    _ = list.pop_front();
    assert!(list.eq_to_iter_refs(&[1, 2, 3, 4]));
    assert_eq!(list.try_get(&idx1), Ok(&1));
    assert_eq!(list.try_get(&idx4), Ok(&4));

    *list.try_get_mut(&idx1).unwrap() = 42;
    assert_eq!(list.try_get(&idx1), Ok(&42));

    assert_eq!(list.node_utilization().num_active_nodes, 4);
    assert_eq!(list.node_utilization().num_closed_nodes, 1);

    list.reclaim_closed_nodes();

    assert_eq!(
        list.try_get(&idx1),
        Err(NodeIdxError::ReorganizedCollection)
    );
    assert_eq!(list.try_get(&idx4), Err(NodeIdxError::OutOfBounds));
}
