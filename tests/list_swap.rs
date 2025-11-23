mod doubly;

use orx_linked_list::*;
use rand::Rng;

#[test]
fn list_swap() {
    let n = 50;
    let mut r = doubly::rng();

    let mut list = DoublyList::new();
    for i in 0..n {
        match i % 3 == 0 {
            true => list.push_front(i.to_string()),
            false => list.push_back(i.to_string()),
        };
    }
    let idx: Vec<_> = list.indices().collect();
    let mut control: Vec<_> = list.iter().cloned().collect();

    for _ in 0..100 {
        let a = r.random_range(0..list.len());
        let b = r.random_range(0..list.len());

        list.swap(idx[a], idx[b]);

        // validate
        #[cfg(feature = "validation")]
        list.validate();

        let val_a = list.get(idx[a]).unwrap();
        let val_b = list.get(idx[b]).unwrap();

        let idx_a = control.iter().position(|x| x == val_a).unwrap();
        let idx_b = control.iter().position(|x| x == val_b).unwrap();

        control.swap(idx_a, idx_b);

        assert_eq!(&control, &list.iter().cloned().collect::<Vec<_>>());
    }
}
