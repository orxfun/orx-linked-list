use orx_linked_list::*;

#[test]
fn iter_from_singly() {
    let mut list = SinglyList::new();
    let mut idx42 = None;
    for i in 0..324 {
        let idx = list.push_front(i.to_string());
        if i == 42 {
            idx42 = Some(idx)
        }
    }

    let idx42 = idx42.unwrap();

    let vec: Vec<_> = list.iter().cloned().collect();
    let index_of_42 = vec
        .iter()
        .enumerate()
        .find(|(_, x)| x == &&42.to_string())
        .unwrap()
        .0;

    let vec_slice = &vec[index_of_42..];
    let mut list_slice = list.iter_from(&idx42);
    list.validate();

    for x in vec_slice {
        let value = list_slice.next().unwrap();
        assert_eq!(x, value);
    }

    assert!(list_slice.next().is_none());
}

#[test]
fn iter_from_doubly() {
    let mut list = DoublyList::new();
    let mut idx42 = None;
    for i in 0..324 {
        let idx = match i % 3 == 0 {
            true => list.push_back(i.to_string()),
            false => list.push_front(i.to_string()),
        };

        if i == 42 {
            idx42 = Some(idx)
        }
    }

    let idx42 = idx42.unwrap();

    let vec: Vec<_> = list.iter().cloned().collect();
    let index_of_42 = vec
        .iter()
        .enumerate()
        .find(|(_, x)| x == &&42.to_string())
        .unwrap()
        .0;

    let vec_slice = &vec[index_of_42..];
    let mut list_slice = list.iter_from(&idx42);
    list.validate();

    for x in vec_slice {
        let value = list_slice.next().unwrap();
        assert_eq!(x, value);
    }

    assert!(list_slice.next().is_none());
}

#[test]
fn iter_backward_from_doubly() {
    let mut list = DoublyList::new();
    let mut idx42 = None;
    for i in 0..324 {
        let idx = match i % 3 == 0 {
            true => list.push_back(i.to_string()),
            false => list.push_front(i.to_string()),
        };

        if i == 42 {
            idx42 = Some(idx)
        }
    }

    let idx42 = idx42.unwrap();

    let vec: Vec<_> = list.iter().cloned().collect();
    let index_of_42 = vec
        .iter()
        .enumerate()
        .find(|(_, x)| x == &&42.to_string())
        .unwrap()
        .0;

    let vec_slice = &vec[0..=index_of_42];
    let mut list_slice = list.iter_backward_from(&idx42);
    list.validate();

    for x in vec_slice.iter().rev() {
        let value = list_slice.next().unwrap();
        assert_eq!(x, value);
    }

    assert!(list_slice.next().is_none());
}
