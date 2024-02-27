use orx_linked_list::{Doubly, List, Singly};

const LARGE_LEN: usize = 1024 * 16;

#[test]
fn large_list_singly() {
    let mut list: List<Singly, _> = List::new();

    for i in 0..LARGE_LEN {
        list.push_front(LARGE_LEN - 1 - i);
    }

    for _ in 0..(LARGE_LEN / 2) {
        list.pop_front();
    }

    for i in (LARGE_LEN / 2)..LARGE_LEN {
        list.push_front(LARGE_LEN - 1 - i);
    }

    for (i, x) in list.iter().enumerate() {
        assert_eq!(i, *x);
    }

    assert_eq!(LARGE_LEN, list.iter().len());

    for i in 0..LARGE_LEN {
        let x = list.pop_front();
        let x = x.expect("must be some");
        assert_eq!(i, x);
    }

    assert!(list.is_empty());
}

#[test]
fn large_list_doubly() {
    let mut list: List<Doubly, _> = List::new();

    for i in 0..LARGE_LEN {
        list.push_back(i);
    }

    for _ in 0..(LARGE_LEN / 2) {
        list.pop_back();
    }

    for i in (LARGE_LEN / 2)..LARGE_LEN {
        list.push_back(i);
    }

    for (i, x) in list.iter().enumerate() {
        assert_eq!(i, *x);
    }
    for (i, x) in list.iter_from_back().enumerate() {
        assert_eq!(LARGE_LEN - 1 - i, *x);
    }

    assert_eq!(LARGE_LEN, list.iter().len());
    assert_eq!(LARGE_LEN, list.iter_from_back().len());

    for i in 0..LARGE_LEN {
        let x = list.pop_front();
        let x = x.expect("must be some");
        assert_eq!(i, x);
    }

    assert!(list.is_empty());
}
