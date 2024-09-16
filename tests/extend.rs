mod doubly;

use orx_linked_list::*;

#[test]
fn extend_doubly() {
    let mut list = doubly::new_doubly(&mut doubly::rng(), 20, 30);
    let mut expected: Vec<_> = list.iter().cloned().collect();

    let mut second: Vec<_> = (0..11).map(|i| i.to_string()).collect();
    list.extend(second.clone());
    expected.append(&mut second);

    let mut third: Vec<_> = (0..7).map(|i| (42 + i).to_string()).collect();
    list.extend(&third);
    expected.append(&mut third);

    assert!(list.eq_to_iter_vals(expected));
}
