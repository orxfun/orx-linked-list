mod doubly;

use orx_linked_list::*;

#[test]
fn ring_iter_mut_on_list() {
    let mut list = doubly::new_doubly(&mut doubly::rng(), 40, 40);
    let n = list.len();

    let idx: Vec<_> = list.indices().collect();
    let values: Vec<_> = list.iter().cloned().collect();

    for i in 0..n {
        let cyclic: Vec<_> = list.ring_iter_mut(idx[i]).map(|x| x.clone()).collect();
        assert_eq!(cyclic.len(), n);

        for j in i..n {
            assert_eq!(cyclic[j - i], values[j]);
        }
        for j in 0..i {
            assert_eq!(cyclic[j + n - i], values[j]);
        }
    }
}

#[test]
fn ring_iter_mut_on_slice() {
    let n = 40;
    let a = 7;
    let b = 23;

    let mut list = doubly::new_doubly(&mut doubly::rng(), n, 40);
    let idx: Vec<_> = list.indices().collect();

    let n = b - a;
    let mut slice = list.slice_mut(idx[a]..idx[b]);

    let idx: Vec<_> = slice.indices().collect();
    let values: Vec<_> = slice.iter().cloned().collect();

    for i in 0..n {
        let cyclic: Vec<_> = slice.ring_iter_mut(idx[i]).map(|x| x.clone()).collect();
        assert_eq!(cyclic.len(), n);

        for j in i..n {
            assert_eq!(cyclic[j - i], values[j]);
        }
        for j in 0..i {
            assert_eq!(cyclic[j + n - i], values[j]);
        }
    }
}

#[test]
fn ring_iter_mut_demo() {
    // a simple scan impl
    fn scan<'a, I: Iterator<Item = &'a mut i32>>(mut values: I) {
        if let Some(first) = values.next() {
            let mut acc = *first;
            for x in values {
                let new_acc = acc + *x;
                *x += acc;
                acc = new_acc;
            }
        }
    }

    // regular scan
    let mut list: DoublyList<_> = (0..5).collect();
    scan(list.iter_mut());
    assert!(list.eq_to_iter_vals([0, 1, 3, 6, 10]));

    // circular scan starting from a pivot point in the middle
    let mut list: DoublyList<_> = (0..5).collect();
    let idx: Vec<_> = list.indices().collect();
    scan(list.ring_iter_mut(idx[3]));
    assert!(list.eq_to_iter_vals([7, 8, 10, 3, 7]));
}
