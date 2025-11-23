mod doubly;

use orx_linked_list::*;

#[test]
fn list_reverse() {
    let mut list = doubly::new_doubly(&mut doubly::rng(), 100, 300);
    let expected: Vec<_> = list.iter().rev().cloned().collect();

    list.reverse();
    assert!(list.eq_to_iter_refs(&expected));
}

#[test]
fn slice_reverse_empty() {
    for i in 0..20 {
        let mut list = doubly::new_doubly(&mut &mut doubly::rng_with_seed(100 * i as u64), 20, 50);
        let expected: Vec<_> = list.iter().cloned().collect();
        let idx: Vec<_> = list.indices().collect();

        let j = i;

        if i < idx.len() {
            list.slice_mut(idx[i]..idx[j]).reverse();
            assert!(list.eq_to_iter_refs(&expected));
        }
    }
}

#[test]
fn slice_reverse_single() {
    for i in 0..20 {
        let mut list = doubly::new_doubly(&mut &mut doubly::rng_with_seed(100 * i as u64), 20, 50);
        let expected: Vec<_> = list.iter().cloned().collect();
        let idx: Vec<_> = list.indices().collect();

        let j = i;

        if i < idx.len() {
            list.slice_mut(idx[i]..=idx[j]).reverse();
            assert!(list.eq_to_iter_refs(&expected));
        }
    }
}

#[test]
fn slice_reverse_from_front() {
    for i in 0..20 {
        let mut list = doubly::new_doubly(&mut &mut doubly::rng_with_seed(100 * i as u64), 20, 50);
        let mut expected: Vec<_> = list.iter().cloned().collect();
        let idx: Vec<_> = list.indices().collect();

        let j = i;
        let i = 0;

        if j < idx.len() {
            expected[0..=j].reverse();

            list.slice_mut(idx[i]..=idx[j]).reverse();
            assert!(list.eq_to_iter_refs(&expected));
        }
    }
}

#[test]
fn slice_reverse_until_back() {
    for i in 0..20 {
        let mut list = doubly::new_doubly(&mut &mut doubly::rng_with_seed(100 * i as u64), 20, 50);
        let mut expected: Vec<_> = list.iter().cloned().collect();
        let idx: Vec<_> = list.indices().collect();

        let j = i;
        let i = list.len();

        if i < list.len() && j < list.len() {
            expected[i..=j].reverse();

            list.slice_mut(idx[i]..=idx[j]).reverse();
            assert!(list.eq_to_iter_refs(&expected));
        }
    }
}

#[test]
fn slice_reverse_middle() {
    for i in 0..20 {
        let mut list = doubly::new_doubly(&mut &mut doubly::rng_with_seed(100 * i as u64), 20, 50);
        let mut expected: Vec<_> = list.iter().cloned().collect();
        let idx: Vec<_> = list.indices().collect();

        let j = i + 3;

        if i < list.len() && j < list.len() {
            expected[i..=j].reverse();

            list.slice_mut(idx[i]..=idx[j]).reverse();
            assert!(list.eq_to_iter_refs(&expected));
        }
    }
}
