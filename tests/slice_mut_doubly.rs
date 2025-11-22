mod doubly;
use orx_linked_list::*;
use std::fmt::Debug;

fn assert_empty_slice<'a, T: Debug + Eq>(slice: &DoublyListSliceMut<'a, T>) {
    assert_eq!(slice.iter().count(), 0);
    assert_eq!(slice.iter_ptr().count(), 0);
    assert_eq!(slice.front(), None);
    assert_eq!(slice.back(), None);
}

#[test]
fn empty_from_empty() {
    let mut list = DoublyList::<usize>::new();
    assert_empty_slice(&list.slice_mut(..));
}

#[test]
fn empty_from_nonempty() {
    let mut list = DoublyList::new();
    let b = list.push_back('b');
    let a = list.push_front('a');
    let c = list.push_back('c');
    assert!(list.eq_to_iter_vals(['a', 'b', 'c']));

    let indices = [a.clone(), b.clone(), c.clone()];

    for x in &indices {
        assert_empty_slice(&list.slice_mut(x..x));
    }
}

#[test]
fn singleton_slice() {
    let mut list = DoublyList::new();
    let b = list.push_back('b');
    let a = list.push_front('a');
    let c = list.push_back('c');

    let expected = ['x', 'y', 'z'];

    let indices = [a.clone(), b.clone(), c.clone()];

    for (i, x) in indices.iter().copied().enumerate() {
        let mut slice = list.slice_mut(x..=x);
        *slice.get_mut(x).unwrap() = expected[i];
    }

    assert!(list.eq_to_iter_refs(expected.iter()));
}

#[test]
fn full_slice() {
    let mut list = DoublyList::new();
    list.push_back('b');
    list.push_front('a');
    list.push_back('c');

    let expected = ['x', 'y', 'z'];
    for (i, x) in list.slice_mut(..).iter_mut().enumerate() {
        *x = expected[i];
    }

    assert!(list.eq_to_iter_refs(expected.iter()));
}
