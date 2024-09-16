use orx_linked_list::*;

#[test]
fn idx_of_singly() {
    let mut list = SinglyList::new();
    list.push_front(3);
    list.push_front(1);
    let idx = list.push_front(42);
    list.push_front(7);

    assert_eq!(list.idx_of(&42).as_ref(), Some(&idx));
    assert!(list.contains(&42));
    assert_eq!(list.position_of_value(&42), Some(1));

    _ = list.pop_front();

    assert_eq!(list.idx_of(&42).as_ref(), Some(&idx));
    assert!(list.contains(&42));
    assert_eq!(list.position_of_value(&42), Some(0));

    let popped = list.pop_front();
    assert_eq!(popped, Some(42));

    assert_eq!(list.idx_of(&42).as_ref(), None);
    assert!(!list.contains(&42));
    assert_eq!(list.position_of_value(&42), None);
}

#[test]
fn idx_of_doubly() {
    let mut list = DoublyList::new();
    list.push_back(3);
    list.push_front(1);
    let idx = list.push_back(42);
    list.push_back(7);

    assert_eq!(list.idx_of(&42).as_ref(), Some(&idx));
    assert!(list.contains(&42));
    assert_eq!(list.position_of_value(&42), Some(2));

    _ = list.pop_back();

    assert_eq!(list.idx_of(&42).as_ref(), Some(&idx));
    assert!(list.contains(&42));
    assert_eq!(list.position_of_value(&42), Some(2));

    let popped = list.pop_back();
    assert_eq!(popped, Some(42));

    assert_eq!(list.idx_of(&42).as_ref(), None);
    assert!(!list.contains(&42));
    assert_eq!(list.position_of_value(&42), None);
}
