use orx_linked_list::*;
use test_case::test_matrix;

#[test_matrix([
    SinglyList::from_iter(['b', 'c', 'd']),
    SinglyListLazy::from_iter(['b', 'c', 'd']),
])]
fn insert_at_singly<M: MemoryPolicy<Singly<char>>>(mut list: List<Singly<char>, M>) {
    list.insert_at(0, 'a');
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd']));

    list.insert_at(4, 'e');
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd', 'e']));

    list.insert_at(3, 'x');
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'x', 'd', 'e']));
}

#[should_panic]
#[test]
fn insert_at_singly_oob() {
    let mut list = SinglyList::from_iter(['b', 'c', 'd']);
    let _ = list.insert_at(4, 'x');
}

#[test_matrix([
    DoublyList::from_iter(['b', 'c', 'd']),
    DoublyListLazy::from_iter(['b', 'c', 'd']),
])]
fn insert_at_doubly<M: MemoryPolicy<Doubly<char>>>(mut list: List<Doubly<char>, M>) {
    list.insert_at(0, 'a');
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd']));

    list.insert_at(4, 'e');
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd', 'e']));

    list.insert_at(3, 'x');
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'x', 'd', 'e']));
}

#[should_panic]
#[test]
fn insert_at_doubly_oob() {
    let mut list = DoublyList::from_iter(['b', 'c', 'd']);
    let _ = list.insert_at(4, 'x');
}

#[test_matrix([
    DoublyList::from_iter(['b', 'c', 'd']),
    DoublyListLazy::from_iter(['b', 'c', 'd']),
])]
fn insert_at_from_back_doubly<M: MemoryPolicy<Doubly<char>>>(mut list: List<Doubly<char>, M>) {
    list.insert_at_from_back(0, 'e');
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['b', 'c', 'd', 'e']));

    list.insert_at_from_back(4, 'a');
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd', 'e']));

    list.insert_at_from_back(2, 'x');
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'x', 'd', 'e']));
}

#[should_panic]
#[test]
fn insert_at_from_back_doubly_oob() {
    let mut list = DoublyList::from_iter(['b', 'c', 'd']);
    let _ = list.insert_at_from_back(4, 'x');
}
