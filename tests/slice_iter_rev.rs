use orx_linked_list::*;

#[test]
fn iter_rev_on_slice() {
    let list: DoublyList<_> = (0..10).collect();
    let idx: Vec<_> = list.indices().collect();

    let slice = list.slice(idx[3]..idx[7]);
    let rev: Vec<_> = slice.iter().copied().rev().collect();
    assert_eq!(rev, [6, 5, 4, 3]);

    let slice = list.slice(idx[3]..idx[2]);
    let rev: Vec<_> = slice.iter().copied().rev().collect();
    assert_eq!(rev, [1, 0]);

    let slice = list.slice(idx[0]..=idx[9]);
    let rev: Vec<_> = slice.iter().copied().rev().collect();
    assert_eq!(rev, [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);

    let slice = list.slice(idx[4]..=idx[4]);
    let rev: Vec<_> = slice.iter().copied().rev().collect();
    assert_eq!(rev, [4]);

    let slice = list.slice(idx[9]..=idx[9]);
    let rev: Vec<_> = slice.iter().copied().rev().collect();
    assert_eq!(rev, [9]);

    let slice = list.slice(idx[0]..idx[0]);
    let rev: Vec<_> = slice.iter().copied().rev().collect();
    assert_eq!(rev, []);
}
