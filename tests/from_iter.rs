use orx_linked_list::*;
use test_case::test_matrix;

#[test_matrix([0, 17, 200])]
fn from_iter_singly(len: usize) {
    let iter = (0..len).map(|i| i.to_string());
    let list: SinglyList<_> = iter.collect();
    #[cfg(feature = "validation")]
    list.validate();

    match len {
        0 => {
            assert!(list.is_empty());
            assert_eq!(list.front(), None);
        }
        _ => {
            assert_eq!(list.len(), len);
            assert_eq!(list.front(), Some(&0.to_string()));

            for (i, x) in list.iter().enumerate() {
                assert_eq!(x, &i.to_string());
            }
        }
    }
}

#[test_matrix([0, 17, 200])]
fn from_iter_doubly(len: usize) {
    let iter = (0..len).map(|i| i.to_string());
    let list: DoublyList<_> = iter.collect();
    #[cfg(feature = "validation")]
    list.validate();

    match len {
        0 => {
            assert!(list.is_empty());
            assert_eq!(list.front(), None);
            assert_eq!(list.back(), None);
        }
        _ => {
            assert_eq!(list.len(), len);
            assert_eq!(list.front(), Some(&0.to_string()));
            assert_eq!(list.back(), Some(&(len - 1).to_string()));

            for (i, x) in list.iter().enumerate() {
                assert_eq!(x, &i.to_string());
            }

            for (i, x) in list.iter().rev().enumerate() {
                assert_eq!(x, &(len - i - 1).to_string());
            }
        }
    }
}
