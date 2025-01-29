use orx_linked_list::*;
use test_case::test_matrix;

#[test_matrix([
    SinglyList::from_iter(['a', 'b', 'c', 'd', 'e']),
    SinglyListLazy::from_iter(['a', 'b', 'c', 'd', 'e']),
])]
fn remove_at_singly<M: MemoryPolicy<Singly<char>>>(mut list: List<Singly<char>, M>) {
    let value = list.remove_at(0);
    assert_eq!(value, Some('a'));
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['b', 'c', 'd', 'e']));

    let value = list.remove_at(3);
    assert_eq!(value, Some('e'));
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['b', 'c', 'd']));

    let value = list.remove_at(1);
    assert_eq!(value, Some('c'));
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['b', 'd']));
}

#[test]
fn remove_at_singly_oob() {
    let mut list = SinglyList::from_iter(['b', 'c', 'd']);
    assert_eq!(list.remove_at(3), None);
}

#[test_matrix([
    DoublyList::from_iter(['a', 'b', 'c', 'd', 'e']),
    DoublyListLazy::from_iter(['a', 'b', 'c', 'd', 'e']),
])]
fn remove_at_doubly<M: MemoryPolicy<Doubly<char>>>(mut list: List<Doubly<char>, M>) {
    let value = list.remove_at(0);
    assert_eq!(value, Some('a'));
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['b', 'c', 'd', 'e']));

    let value = list.remove_at(3);
    assert_eq!(value, Some('e'));
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['b', 'c', 'd']));

    let value = list.remove_at(1);
    assert_eq!(value, Some('c'));
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['b', 'd']));
}

#[test]
fn remove_at_doubly_oob() {
    let mut list = DoublyList::from_iter(['b', 'c', 'd']);
    assert_eq!(list.remove_at(3), None);
}

#[test_matrix([
    DoublyList::from_iter(['a', 'b', 'c', 'd', 'e']),
    DoublyListLazy::from_iter(['a', 'b', 'c', 'd', 'e']),
])]
fn remove_at_from_back_doubly<M: MemoryPolicy<Doubly<char>>>(mut list: List<Doubly<char>, M>) {
    let value = list.remove_at_from_back(4);
    assert_eq!(value, Some('a'));
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['b', 'c', 'd', 'e']));

    let value = list.remove_at_from_back(0);
    assert_eq!(value, Some('e'));
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['b', 'c', 'd']));

    let value = list.remove_at_from_back(1);
    assert_eq!(value, Some('c'));
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['b', 'd']));
}

#[test]
fn remove_at_from_back_doubly_oob() {
    let mut list = DoublyList::from_iter(['b', 'c', 'd']);
    assert_eq!(list.remove_at_from_back(3), None);
}
