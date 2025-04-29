use orx_linked_list::*;

fn test_singly_iter_from(
    list: &SinglyList<String>,
    vec: &[String],
    value: usize,
    idx: NodeIdx<Singly<String>>,
) {
    let position = vec
        .iter()
        .enumerate()
        .find(|(_, x)| x == &&value.to_string())
        .unwrap()
        .0;

    let vec_slice = &vec[position..];
    let mut list_slice = list.iter_from(&idx);
    #[cfg(feature = "validation")]
    list.validate();

    for x in vec_slice {
        let value = list_slice.next().unwrap();
        assert_eq!(x, value);
    }

    assert!(list_slice.next().is_none());
}

fn test_doubly_iter_from(
    list: &DoublyList<String>,
    vec: &[String],
    value: usize,
    idx: NodeIdx<Doubly<String>>,
) {
    let position = vec
        .iter()
        .enumerate()
        .find(|(_, x)| x == &&value.to_string())
        .unwrap()
        .0;

    let vec_slice = &vec[position..];
    let mut list_slice = list.iter_from(&idx);
    #[cfg(feature = "validation")]
    list.validate();

    for x in vec_slice {
        let value = list_slice.next().unwrap();
        assert_eq!(x, value);
    }

    assert!(list_slice.next().is_none());
}

fn test_doubly_iter_backward_from(
    list: &DoublyList<String>,
    vec: &[String],
    value: usize,
    idx: NodeIdx<Doubly<String>>,
) {
    let position = vec
        .iter()
        .enumerate()
        .find(|(_, x)| x == &&value.to_string())
        .unwrap()
        .0;

    let vec_slice = &vec[0..=position];
    let mut list_slice = list.iter_backward_from(&idx);
    #[cfg(feature = "validation")]
    list.validate();

    for x in vec_slice.iter().rev() {
        let value = list_slice.next().unwrap();
        assert_eq!(x, value);
    }

    assert!(list_slice.next().is_none());
}

#[test]
fn iter_from_singly() {
    let mut list = SinglyList::new();
    let [mut idx_first, mut idx_last, mut idx42] = [None, None, None];
    for i in 0..324 {
        let idx = list.push_front(i.to_string());
        match i {
            0 => idx_first = Some(idx),
            42 => idx42 = Some(idx),
            323 => idx_last = Some(idx),
            _ => {}
        };
    }

    let vec: Vec<_> = list.iter().cloned().collect();

    test_singly_iter_from(&list, &vec, 0, idx_first.unwrap());
    test_singly_iter_from(&list, &vec, 323, idx_last.unwrap());
    test_singly_iter_from(&list, &vec, 42, idx42.unwrap());
}

#[test]
fn iter_from_doubly() {
    let mut list = DoublyList::new();
    let [mut idx_first, mut idx_last, mut idx42] = [None, None, None];
    for i in 0..324 {
        let idx = match i % 3 == 0 {
            true => list.push_back(i.to_string()),
            false => list.push_front(i.to_string()),
        };

        match i {
            0 => idx_first = Some(idx),
            42 => idx42 = Some(idx),
            323 => idx_last = Some(idx),
            _ => {}
        };
    }

    let vec: Vec<_> = list.iter().cloned().collect();

    test_doubly_iter_from(&list, &vec, 0, idx_first.unwrap());
    test_doubly_iter_from(&list, &vec, 323, idx_last.unwrap());
    test_doubly_iter_from(&list, &vec, 42, idx42.unwrap());
}

#[test]
fn iter_backward_from_doubly() {
    let mut list = DoublyList::new();
    let [mut idx_first, mut idx_last, mut idx42] = [None, None, None];
    for i in 0..324 {
        let idx = match i % 3 == 0 {
            true => list.push_back(i.to_string()),
            false => list.push_front(i.to_string()),
        };

        match i {
            0 => idx_first = Some(idx),
            42 => idx42 = Some(idx),
            323 => idx_last = Some(idx),
            _ => {}
        };
    }

    let vec: Vec<_> = list.iter().cloned().collect();

    test_doubly_iter_backward_from(&list, &vec, 0, idx_first.unwrap());
    test_doubly_iter_backward_from(&list, &vec, 323, idx_last.unwrap());
    test_doubly_iter_backward_from(&list, &vec, 42, idx42.unwrap());
}
