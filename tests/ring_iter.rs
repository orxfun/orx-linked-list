mod doubly;

use orx_linked_list::*;

#[test]
fn ring_iter_on_list() {
    let list = doubly::new_doubly(&mut doubly::rng(), 40, 40);
    let idx: Vec<_> = list.indices().collect();
    let values: Vec<_> = list.iter().collect();

    for i in 0..list.len() {
        let cyclic: Vec<_> = list.ring_iter(idx[i]).collect();
        assert_eq!(cyclic.len(), list.len());

        for j in i..list.len() {
            assert_eq!(cyclic[j - i], values[j]);
        }
        for j in 0..i {
            assert_eq!(cyclic[j + list.len() - i], values[j]);
        }

        let mut cyclic_rev: Vec<_> = list.ring_iter(idx[i]).rev().collect();
        cyclic_rev.reverse();
        assert_eq!(cyclic, cyclic_rev);
    }
}

#[test]
fn ring_iter_on_slice() {
    let n = 40;
    let a = 7;
    let b = 23;

    let list = doubly::new_doubly(&mut doubly::rng(), n, 40);
    let idx: Vec<_> = list.indices().collect();

    let n = b - a;
    let slice = list.slice(idx[a]..idx[b]);

    let idx: Vec<_> = slice.indices().collect();
    let values: Vec<_> = slice.iter().collect();

    for i in 0..n {
        let cyclic: Vec<_> = slice.ring_iter(idx[i]).collect();
        assert_eq!(cyclic.len(), n);

        for j in i..n {
            assert_eq!(cyclic[j - i], values[j]);
        }
        for j in 0..i {
            assert_eq!(cyclic[j + n - i], values[j]);
        }

        let mut cyclic_rev: Vec<_> = slice.ring_iter(idx[i]).rev().collect();

        cyclic_rev.reverse();
        assert_eq!(cyclic, cyclic_rev);
    }
}

#[test]
fn ring_iter_demo() {
    let list: DoublyList<_> = (0..8).collect();
    let idx: Vec<_> = list.indices().collect();

    let iter = list.ring_iter(idx[2]);
    assert_eq!(iter.copied().collect::<Vec<_>>(), [2, 3, 4, 5, 6, 7, 0, 1]);

    let iter = list.ring_iter(idx[4]);
    assert_eq!(iter.copied().collect::<Vec<_>>(), [4, 5, 6, 7, 0, 1, 2, 3]);

    // ring iterator is also double-ended
    let iter = list.ring_iter(idx[4]).rev();
    assert_eq!(iter.copied().collect::<Vec<_>>(), [3, 2, 1, 0, 7, 6, 5, 4]);

    // ring iterators are also available for slices
    let slice = list.slice(idx[3]..idx[7]);
    assert!(slice.eq_to_iter_vals([3, 4, 5, 6]));

    let iter = slice.ring_iter(idx[4]);
    assert_eq!(iter.copied().collect::<Vec<_>>(), [4, 5, 6, 3,]);

    let iter = slice.ring_iter(idx[6]);
    assert_eq!(iter.copied().collect::<Vec<_>>(), [6, 3, 4, 5]);
}
