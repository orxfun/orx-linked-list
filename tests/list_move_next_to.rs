use orx_linked_list::*;

fn list_and_indices(n: usize) -> (DoublyList<usize>, Vec<DoublyIdx<usize>>) {
    let list: DoublyList<_> = (0..n).collect();
    let indices: Vec<_> = list.indices().collect();
    (list, indices)
}

#[test]
fn list_move_next_to_front() {
    let n = 10;
    for i in 1..n {
        let (mut list, idx) = list_and_indices(n);
        list.move_next_to(&idx[i], &idx[0]);

        let mut vec: Vec<_> = (0..n).into_iter().filter(|x| x != &i).collect();
        vec.insert(1, i);

        #[cfg(feature = "validation")]
        list.validate();
        assert!(list.eq_to_iter_refs(&vec));
    }
}

#[test]
fn list_move_next_to_back() {
    let n = 10;
    for i in 0..n {
        let (mut list, idx) = list_and_indices(n);
        list.move_next_to(&idx[i], &idx[n - 1]);

        let mut vec: Vec<_> = (0..n).into_iter().filter(|x| x != &i).collect();
        vec.insert(n - 1, i);

        dbg!(i, &vec, list.iter().collect::<Vec<_>>());

        #[cfg(feature = "validation")]
        list.validate();
        assert!(list.eq_to_iter_refs(&vec));
    }
}

#[test]
fn list_move_next_front_prev_to_arbitrary() {
    let (mut list, idx) = list_and_indices(5);
    list.move_next_to(&idx[0], &idx[2]);
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals([1, 2, 0, 3, 4]));

    let (mut list, idx) = list_and_indices(5);
    list.move_next_to(&idx[0], &idx[3]);
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals([1, 2, 3, 0, 4]));

    let (mut list, idx) = list_and_indices(5);
    list.move_next_to(&idx[0], &idx[4]);
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals([1, 2, 3, 4, 0]));
}

#[test]
fn list_move_next_back_prev_to_arbitrary() {
    let (mut list, idx) = list_and_indices(5);
    list.move_next_to(&idx[4], &idx[2]);
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals([0, 1, 2, 4, 3]));

    let (mut list, idx) = list_and_indices(5);
    list.move_next_to(&idx[4], &idx[0]);
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals([0, 4, 1, 2, 3]));

    let (mut list, idx) = list_and_indices(5);
    list.move_next_to(&idx[4], &idx[1]);
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals([0, 1, 4, 2, 3]));
}

#[test]
fn list_move_next_to_arbitrary() {
    let (mut list, idx) = list_and_indices(5);
    list.move_next_to(&idx[3], &idx[2]);
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals([0, 1, 2, 3, 4]));

    let (mut list, idx) = list_and_indices(5);
    list.move_next_to(&idx[3], &idx[0]);
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals([0, 3, 1, 2, 4]));

    let (mut list, idx) = list_and_indices(5);
    list.move_next_to(&idx[1], &idx[3]);
    #[cfg(feature = "validation")]
    list.validate();
    assert!(list.eq_to_iter_vals([0, 2, 3, 1, 4]));
}
