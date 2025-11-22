use orx_linked_list::*;

#[test]
fn slice_in_wrong_direction() {
    let list: DoublyList<_> = (0..10).collect();

    let idx: Vec<_> = list.indices().collect();

    let slice = list.slice(idx[1]..idx[4]);
    assert!(slice.eq_to_iter_vals([1, 2, 3]));

    let slice = list.slice(idx[4]..idx[1]);
    assert!(slice.eq_to_iter_vals([4, 5, 6, 7, 8, 9]));

    let slice = list.slice(idx[4]..=idx[9]);
    assert!(slice.eq_to_iter_vals([4, 5, 6, 7, 8, 9]));

    let slice = list.slice(idx[4]..=idx[4]);
    assert!(slice.eq_to_iter_vals([4]));

    let slice = list.slice(idx[4]..idx[4]);
    assert!(slice.eq_to_iter_vals([]));
}
