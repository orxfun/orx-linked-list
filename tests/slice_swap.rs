use orx_linked_list::*;

#[test]
fn slice_swap_with_all_idx() {
    let mut list: DoublyList<_> = (0..7).collect();
    let all_idx: Vec<_> = list.indices().collect();

    let mut slice = list.slice_mut(&all_idx[1]..&all_idx[5]);
    assert_eq!(slice.iter().copied().collect::<Vec<_>>(), [1, 2, 3, 4]);

    slice.swap(&all_idx[1], &all_idx[4]);
    assert_eq!(slice.iter().copied().collect::<Vec<_>>(), [4, 2, 3, 1]);

    slice.swap(&all_idx[2], &all_idx[1]);
    assert_eq!(slice.iter().copied().collect::<Vec<_>>(), [4, 1, 3, 2]);

    slice.swap(&all_idx[2], &all_idx[3]);
    assert_eq!(slice.iter().copied().collect::<Vec<_>>(), [4, 1, 2, 3]);

    #[cfg(feature = "validation")]
    list.validate();

    assert_eq!(
        list.iter().copied().collect::<Vec<_>>(),
        [0, 4, 1, 2, 3, 5, 6]
    );
}

#[test]
fn slice_swap_with_slice_idx() {
    let mut list: DoublyList<_> = (0..7).collect();
    let all_idx: Vec<_> = list.indices().collect();

    let mut slice = list.slice_mut(&all_idx[1]..&all_idx[5]);

    let idx: Vec<_> = slice.indices().collect();
    assert_eq!(slice.iter().copied().collect::<Vec<_>>(), [1, 2, 3, 4]);

    slice.swap(&idx[1], &idx[3]);
    assert_eq!(slice.iter().copied().collect::<Vec<_>>(), [1, 4, 3, 2]);

    slice.swap(&idx[0], &idx[2]);
    assert_eq!(slice.iter().copied().collect::<Vec<_>>(), [3, 4, 1, 2]);

    slice.swap(&idx[3], &idx[1]);
    assert_eq!(slice.iter().copied().collect::<Vec<_>>(), [3, 2, 1, 4]);

    #[cfg(feature = "validation")]
    list.validate();
    assert_eq!(
        list.iter().copied().collect::<Vec<_>>(),
        [0, 3, 2, 1, 4, 5, 6]
    );
}

#[test]
fn slice_swap_with_entire_list() {
    let mut list: DoublyList<_> = (0..7).collect();
    let all_idx: Vec<_> = list.indices().collect();

    let mut slice = list.slice_mut(..);
    assert_eq!(
        slice.iter().copied().collect::<Vec<_>>(),
        [0, 1, 2, 3, 4, 5, 6]
    );

    slice.swap(&all_idx[1], &all_idx[4]);
    assert_eq!(
        slice.iter().copied().collect::<Vec<_>>(),
        [0, 4, 2, 3, 1, 5, 6]
    );

    slice.swap(&all_idx[2], &all_idx[1]);
    assert_eq!(
        slice.iter().copied().collect::<Vec<_>>(),
        [0, 4, 1, 3, 2, 5, 6]
    );

    slice.swap(&all_idx[2], &all_idx[3]);
    assert_eq!(
        slice.iter().copied().collect::<Vec<_>>(),
        [0, 4, 1, 2, 3, 5, 6]
    );

    slice.swap(&all_idx[0], &all_idx[6]);
    assert_eq!(
        slice.iter().copied().collect::<Vec<_>>(),
        [6, 4, 1, 2, 3, 5, 0]
    );

    slice.swap(&all_idx[4], &all_idx[6]);
    assert_eq!(
        slice.iter().copied().collect::<Vec<_>>(),
        [4, 6, 1, 2, 3, 5, 0]
    );

    slice.swap(&all_idx[0], &all_idx[2]);
    assert_eq!(
        slice.iter().copied().collect::<Vec<_>>(),
        [4, 6, 1, 0, 3, 5, 2]
    );

    #[cfg(feature = "validation")]
    list.validate();
    assert_eq!(
        list.iter().copied().collect::<Vec<_>>(),
        [4, 6, 1, 0, 3, 5, 2]
    );
}
