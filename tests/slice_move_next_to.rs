use orx_linked_list::*;

fn list_and_indices(n: usize) -> (DoublyList<usize>, Vec<DoublyIdx<usize>>) {
    let list: DoublyList<_> = (0..n).collect();
    let indices: Vec<_> = list.indices().collect();
    (list, indices)
}

#[test]
fn slice_move_next_to_front() {
    let n = 10;
    let a = 1;
    let b = 5;
    for i in (a + 1)..=b {
        let (mut list, idx) = list_and_indices(n);
        let mut slice = list.slice_mut(&idx[a]..=&idx[b]);
        let idx: Vec<_> = slice.indices().collect();

        slice.move_next_to(&idx[i - a], &idx[0]);

        let slice: Vec<_> = slice.iter().copied().collect();

        list.validate();

        let mut vec: Vec<_> = (0..n).into_iter().filter(|x| x != &i).collect();
        vec.insert(a + 1, i);

        assert_eq!(slice, &vec[a..=b]);
        assert!(list.eq_to_iter_refs(&vec));
    }
}

#[test]
fn slice_move_next_to_back() {
    let n = 10;
    let a = 1;
    let b = 5;
    for i in a..=b {
        let (mut list, idx) = list_and_indices(n);
        let mut slice = list.slice_mut(&idx[a]..=&idx[b]);
        let idx: Vec<_> = slice.indices().collect();

        slice.move_next_to(&idx[i - a], &idx[b - 1]);

        let slice: Vec<_> = slice.iter().copied().collect();

        list.validate();

        let mut vec: Vec<_> = (0..n).into_iter().filter(|x| x != &i).collect();
        vec.insert(b, i);

        assert_eq!(slice, &vec[a..=b]);
        assert!(list.eq_to_iter_refs(&vec));
    }
}

#[test]
fn slice_move_next_to_front_arbitrary() {
    let n = 10;
    let a = 1;
    let b = 5;

    let (mut list, idx) = list_and_indices(n);
    let mut slice = list.slice_mut(&idx[a]..=&idx[b]);
    let idx: Vec<_> = slice.indices().collect();
    slice.move_next_to(&idx[0], &idx[2]);
    assert_eq!(
        slice.iter().copied().collect::<Vec<_>>(),
        vec![2, 3, 1, 4, 5]
    );
    list.validate();
    assert!(list.eq_to_iter_vals([0, 2, 3, 1, 4, 5, 6, 7, 8, 9]));

    let (mut list, idx) = list_and_indices(n);
    let mut slice = list.slice_mut(&idx[a]..=&idx[b]);
    let idx: Vec<_> = slice.indices().collect();
    slice.move_next_to(&idx[0], &idx[3]);
    assert_eq!(
        slice.iter().copied().collect::<Vec<_>>(),
        vec![2, 3, 4, 1, 5]
    );
    list.validate();
    assert!(list.eq_to_iter_vals([0, 2, 3, 4, 1, 5, 6, 7, 8, 9]));

    let (mut list, idx) = list_and_indices(n);
    let mut slice = list.slice_mut(&idx[a]..=&idx[b]);
    let idx: Vec<_> = slice.indices().collect();
    slice.move_next_to(&idx[0], &idx[4]);
    assert_eq!(
        slice.iter().copied().collect::<Vec<_>>(),
        vec![2, 3, 4, 5, 1]
    );
    list.validate();
    assert!(list.eq_to_iter_vals([0, 2, 3, 4, 5, 1, 6, 7, 8, 9]));
}

#[test]
fn slice_move_next_to_back_arbitrary() {
    let n = 10;
    let a = 1;
    let b = 5;

    let (mut list, idx) = list_and_indices(n);
    let mut slice = list.slice_mut(&idx[a]..=&idx[b]);
    let idx: Vec<_> = slice.indices().collect();
    slice.move_next_to(&idx[4], &idx[2]);
    assert_eq!(
        slice.iter().copied().collect::<Vec<_>>(),
        vec![1, 2, 3, 5, 4]
    );
    list.validate();
    assert!(list.eq_to_iter_vals([0, 1, 2, 3, 5, 4, 6, 7, 8, 9]));

    let (mut list, idx) = list_and_indices(n);
    let mut slice = list.slice_mut(&idx[a]..=&idx[b]);
    let idx: Vec<_> = slice.indices().collect();
    slice.move_next_to(&idx[4], &idx[3]);
    assert_eq!(
        slice.iter().copied().collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5]
    );
    list.validate();
    assert!(list.eq_to_iter_vals([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));

    let (mut list, idx) = list_and_indices(n);
    let mut slice = list.slice_mut(&idx[a]..=&idx[b]);
    let idx: Vec<_> = slice.indices().collect();
    slice.move_next_to(&idx[4], &idx[4]);
    assert_eq!(
        slice.iter().copied().collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5]
    );
    list.validate();
    assert!(list.eq_to_iter_vals([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));

    let (mut list, idx) = list_and_indices(n);
    let mut slice = list.slice_mut(&idx[a]..=&idx[b]);
    let idx: Vec<_> = slice.indices().collect();
    slice.move_next_to(&idx[4], &idx[0]);
    assert_eq!(
        slice.iter().copied().collect::<Vec<_>>(),
        vec![1, 5, 2, 3, 4]
    );
    list.validate();
    assert!(list.eq_to_iter_vals([0, 1, 5, 2, 3, 4, 6, 7, 8, 9]));
}

#[test]
fn slice_move_next_to_arbitrary() {
    let n = 10;
    let a = 1;
    let b = 5;

    let (mut list, idx) = list_and_indices(n);
    let mut slice = list.slice_mut(&idx[a]..=&idx[b]);
    let idx: Vec<_> = slice.indices().collect();
    slice.move_next_to(&idx[3], &idx[2]);
    assert_eq!(
        slice.iter().copied().collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5]
    );
    list.validate();
    assert!(list.eq_to_iter_vals([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));

    let (mut list, idx) = list_and_indices(n);
    let mut slice = list.slice_mut(&idx[a]..=&idx[b]);
    let idx: Vec<_> = slice.indices().collect();
    slice.move_next_to(&idx[3], &idx[0]);
    assert_eq!(
        slice.iter().copied().collect::<Vec<_>>(),
        vec![1, 4, 2, 3, 5]
    );
    list.validate();
    assert!(list.eq_to_iter_vals([0, 1, 4, 2, 3, 5, 6, 7, 8, 9]));

    let (mut list, idx) = list_and_indices(n);
    let mut slice = list.slice_mut(&idx[a]..=&idx[b]);
    let idx: Vec<_> = slice.indices().collect();
    slice.move_next_to(&idx[1], &idx[3]);
    assert_eq!(
        slice.iter().copied().collect::<Vec<_>>(),
        vec![1, 3, 4, 2, 5]
    );
    list.validate();
    assert!(list.eq_to_iter_vals([0, 1, 3, 4, 2, 5, 6, 7, 8, 9]));
}