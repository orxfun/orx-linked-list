use orx_linked_list::*;

mod doubly;
mod singly;

#[test]
fn debug_singly() {
    let list: SinglyList<_> = (0..4).collect();
    let str = format!("{:?}", &list);
    assert_eq!(str, "[0 -> 1 -> 2 -> 3]")
}

#[test]
fn debug_doubly() {
    let list: DoublyList<_> = (0..4).collect();
    let str = format!("{:?}", &list);
    assert_eq!(str, "[0 <-> 1 <-> 2 <-> 3]")
}
