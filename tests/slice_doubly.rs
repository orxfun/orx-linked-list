mod doubly;
use orx_linked_list::*;
use std::fmt::Debug;

fn assert_empty_slice<'a, T: Debug + Eq>(slice: &DoublyListSlice<'a, T>) {
    assert_eq!(slice.iter().count(), 0);
    assert_eq!(slice.iter_ptr().count(), 0);
    assert_eq!(slice.front(), None);
    assert_eq!(slice.back(), None);
}

#[test]
fn empty_from_empty() {
    let list = DoublyList::<usize>::new();
    assert_empty_slice(&list.slice(..));
}

#[test]
fn empty_from_nonempty() {
    let mut list = DoublyList::new();
    let b = list.push_back('b');
    let a = list.push_front('a');
    let c = list.push_back('c');
    assert!(list.eq_to_iter_vals(['a', 'b', 'c']));

    let indices = [a, b, c];

    for x in &indices {
        assert_empty_slice(&list.slice(x..x));
    }
}

#[test]
fn singleton_slice() {
    let mut list = DoublyList::new();
    let b = list.push_back('b');
    let a = list.push_front('a');
    let c = list.push_back('c');

    let expected = ['a', 'b', 'c'];
    assert!(list.eq_to_iter_refs(expected.iter()));

    let indices = [a, b, c];

    for (i, x) in indices.iter().enumerate() {
        let slice = list.slice(x..=x);
        slice.eq_to_iter_vals([expected[i]]);
    }
}

#[test]
fn full_slice() {
    let mut list = DoublyList::new();
    let _ = list.push_back('b');
    let a = list.push_front('a');
    let c = list.push_back('c');
    assert!(list.eq_to_iter_vals(['a', 'b', 'c']));

    assert!(list.eq_to_iter_refs(list.slice(..).iter()));
    assert!(list.eq_to_iter_refs(list.slice(&a..).iter()));
    assert!(list.eq_to_iter_refs(list.slice(..=&c).iter()));
}

#[test]
fn non_empty() {
    let mut list = DoublyList::new();
    let b = list.push_back('b');
    let a = list.push_front('a');
    let c = list.push_back('c');

    let expected = ['a', 'b', 'c'];
    assert!(list.eq_to_iter_refs(expected.iter()));

    let indices = [a, b, c];

    for i in 0..expected.len() {
        for j in i..expected.len() {
            let slice = list.slice(&indices[i]..=&indices[j]);
            slice.eq_to_iter_refs(&expected.as_slice()[i..=j]);

            let slice = list.slice(&indices[i]..&indices[j]);
            slice.eq_to_iter_refs(&expected.as_slice()[i..j]);
        }
    }
}

#[test]
fn demo_usage() {
    let mut list = DoublyList::new();

    list.push_back(3);
    list.push_front(1);
    list.push_front(7);
    list.push_back(4);
    list.push_front(9);

    let expected_values = vec![9, 7, 1, 3, 4];

    assert!(list.eq_to_iter_refs(&expected_values));
    assert!(list.slice(..).eq_to_iter_refs(&expected_values));

    let idx: Vec<_> = list.indices().collect();

    let slice = list.slice(idx[1]..=idx[3]);
    assert_eq!(slice.front(), Some(&7));
    assert_eq!(slice.back(), Some(&3));
    assert!(slice.eq_to_iter_vals([7, 1, 3]));

    let sum: usize = slice.iter().sum();
    assert_eq!(sum, 11);
}

#[test]
fn doubly_slice() {
    let list = doubly::new_doubly(&mut doubly::rng(), 50, 200);

    let n = list.len();
    let vec: Vec<_> = list.iter().cloned().collect();
    let idx: Vec<_> = list.indices().collect();

    // empty
    for id in idx.iter().take(n) {
        assert_empty_slice(&list.slice(id..id));
    }

    // single
    for i in 0..n {
        let s = list.slice(idx[i]..=idx[i]);
        s.eq_to_iter_refs(&vec[i..=i]);

        if i != n - 1 {
            let s = list.slice(idx[i]..idx[i + 1]);
            s.eq_to_iter_refs(&vec[i..=i]);
        }
    }

    // full
    list.slice(..).eq_to_iter_refs(&vec[..]);
    list.slice(idx[0]..).eq_to_iter_refs(&vec[0..]);
    list.slice(..=idx[n - 1]).eq_to_iter_refs(&vec[..(n - 1)]);
    list.slice(idx[0]..=idx[n - 1])
        .eq_to_iter_refs(&vec[0..(n - 1)]);

    // arbitrary ranges
    for i in 0..n {
        for j in i..n {
            let s = list.slice(idx[i]..=idx[j]);
            s.eq_to_iter_refs(&vec[i..=j]);

            let s = list.slice(idx[i]..idx[j]);
            s.eq_to_iter_refs(&vec[i..j]);
        }
    }
}
