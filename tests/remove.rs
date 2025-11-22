use orx_linked_list::*;
use test_case::test_matrix;

#[test_matrix(
    [DoublyList::new(), DoublyListLazy::new()],
    [0, 1, 2, 3, 4]
)]
fn remove_doubly<M: MemoryPolicy<Doubly<char>>>(mut list: List<Doubly<char>, M>, idx: usize) {
    let indices = [
        list.push_back('c'),
        list.push_back('d'),
        list.push_front('b'),
        list.push_front('a'),
        list.push_back('e'),
    ];
    let chars = ['c', 'd', 'b', 'a', 'e'];

    let c = chars[idx];

    let idx = indices[idx];
    let removed = list.try_remove(idx);
    #[cfg(feature = "validation")]
    list.validate();

    assert_eq!(removed, Some(c));
}

#[test_matrix([DoublyList::new(), DoublyListLazy::new()])]
fn remove_doubly_oob<M: MemoryPolicy<Doubly<char>>>(mut list: List<Doubly<char>, M>) {
    let indices = [
        list.push_back('c'),
        list.push_back('d'),
        list.push_front('b'),
        list.push_front('a'),
        list.push_back('e'),
    ];

    let _ = list.pop_back();

    let removed = list.try_remove(indices[4]);
    #[cfg(feature = "validation")]
    list.validate();

    assert_eq!(removed, None);
}

#[test_matrix([DoublyList::new(), DoublyListLazy::new()])]
fn remove_doubly_other_list<M: MemoryPolicy<Doubly<char>>>(mut other: List<Doubly<char>, M>) {
    let mut list = DoublyList::new();
    let indices = [
        list.push_back('c'),
        list.push_back('d'),
        list.push_front('b'),
        list.push_front('a'),
        list.push_back('e'),
    ];

    let other_indices = [
        other.push_back('c'),
        other.push_back('d'),
        other.push_front('b'),
        other.push_front('a'),
        other.push_back('e'),
    ];

    let removed = other.try_remove(indices[4]);
    assert_eq!(removed, None);

    let removed = list.try_remove(other_indices[4]);
    assert_eq!(removed, None);

    #[cfg(feature = "validation")]
    list.validate();
    #[cfg(feature = "validation")]
    other.validate();
}
