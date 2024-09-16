use orx_linked_list::*;
use test_case::test_matrix;

#[test_matrix(
    [DoublyList::new(), DoublyListLazy::new()],
    [0, 1, 2, 3, 4],
    [true, false]
)]
fn insert_doubly<M: MemoryPolicy<Doubly<char>>>(
    mut list: List<Doubly<char>, M>,
    idx: usize,
    next_to: bool,
) {
    let indices = [
        list.push_back('c'),
        list.push_back('d'),
        list.push_front('b'),
        list.push_front('a'),
        list.push_back('e'),
    ];

    let mut vec = vec!['a', 'b', 'c', 'd', 'e'];

    assert!(list.eq_to_iter_vals(vec.iter().copied()));

    let at = match next_to {
        true => idx + 1,
        false => idx,
    };
    vec.insert(at, 'f');

    let idx = match idx {
        0 => 3,
        1 => 2,
        2 => 0,
        3 => 1,
        _ => 4,
    };
    let idx = match next_to {
        true => list.try_insert_next_to(&indices[idx], 'f'),
        false => list.try_insert_prev_to(&indices[idx], 'f'),
    }
    .unwrap();
    list.validate();

    assert!(list.eq_to_iter_vals(vec.iter().copied()));
    assert_eq!(list.get(&idx), Some(&'f'));
}

#[test_matrix([0, 1, 2, 3, 4], [true, false])]
fn insert_doubly_oob(idx: usize, next_to: bool) {
    let mut list = DoublyList::new();
    let indices = [
        list.push_back('c'),
        list.push_back('d'),
        list.push_front('b'),
        list.push_front('a'),
        list.push_back('e'),
    ];

    let mut vec = vec!['a', 'b', 'c', 'd', 'e'];
    vec.remove(idx);
    list.validate();

    let idx = match idx {
        0 => 3,
        1 => 2,
        2 => 0,
        3 => 1,
        _ => 4,
    };
    let _ = list.try_remove(&indices[idx]);
    list.validate();
    assert!(list.eq_to_iter_vals(vec.iter().copied()));

    let idx = match next_to {
        true => list.try_insert_next_to(&indices[idx], 'f'),
        false => list.try_insert_prev_to(&indices[idx], 'f'),
    };

    assert!(idx.is_err());
    assert!(list.eq_to_iter_vals(vec.iter().copied()));
}
