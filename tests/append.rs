use orx_linked_list::*;
use test_case::test_matrix;

#[test_matrix(
    [DoublyList::new(), DoublyListLazy::new()]
)]
fn append_front_doubly<M: MemoryPolicy<Doubly<char>>>(mut list: List<Doubly<char>, M>) {
    list.push_back('c');
    list.push_back('d');
    list.push_front('b');
    let a = list.push_front('a');
    list.push_back('e');

    let mut other = DoublyList::new();
    other.push_back('h');
    other.push_back('i');
    let g = other.push_front('g');
    other.push_front('f');
    other.push_back('j');

    list.append_front(other);
    list.validate();
    assert!(list.eq_to_iter_vals(['f', 'g', 'h', 'i', 'j', 'a', 'b', 'c', 'd', 'e']));

    assert_eq!(list.get(&a), Some(&'a'));
    assert_eq!(list.get(&g), Some(&'g'));
}

#[test_matrix(
    [DoublyList::new(), DoublyListLazy::new()]
)]
fn append_back_doubly<M: MemoryPolicy<Doubly<char>>>(mut list: List<Doubly<char>, M>) {
    list.push_back('c');
    list.push_back('d');
    list.push_front('b');
    let a = list.push_front('a');
    list.push_back('e');

    let mut other = DoublyList::new();
    other.push_back('h');
    other.push_back('i');
    let g = other.push_front('g');
    other.push_front('f');
    other.push_back('j');

    list.append_back(other);
    list.validate();
    assert!(list.eq_to_iter_vals(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']));

    assert_eq!(list.get(&a), Some(&'a'));
    assert_eq!(list.get(&g), Some(&'g'));
}
