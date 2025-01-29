use orx_linked_list::*;

mod doubly;
mod singly;

#[test]
fn clone_singly() {
    let list = singly::new_singly(&mut doubly::rng(), 40, 50);
    let clone = list.clone();

    #[cfg(feature = "validation")]
    clone.validate();

    assert!(list.eq_to_iter_refs(clone.iter()));
}

#[test]
fn clone_doubly() {
    let list = doubly::new_doubly(&mut doubly::rng(), 40, 50);
    let clone = list.clone();

    #[cfg(feature = "validation")]
    clone.validate();

    assert!(list.eq_to_iter_refs(clone.iter()));
}
