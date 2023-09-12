#[cfg(test)]
use crate::{node::LinkedListNode, LinkedList, LinkedListX};
#[cfg(test)]
use orx_imp_vec::prelude::{PinnedVec, SelfRefVecItem};

#[cfg(test)]
#[derive(Debug)]
enum SourceNode {
    Prev(usize),
    Next(usize),
    Sentinel,
}

#[cfg(test)]
pub(crate) fn validate_list<'a, T, P>(list: &LinkedList<'a, T, P>)
where
    T: 'a,
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
{
    let veclen = list.vec.len();
    let index_of = |x: &LinkedListNode<'a, T>| list.vec.index_of(x);
    let node = &list.vec[0];
    if let Some(prev) = node.prev() {
        let source_node = SourceNode::Sentinel;
        validate_reference(veclen, index_of, source_node, prev);
    }
    if let Some(next) = node.next() {
        let source_node = SourceNode::Sentinel;
        validate_reference(veclen, index_of, source_node, next);
    }

    for (i, node) in list.vec.iter().enumerate().skip(1) {
        if let Some(prev) = node.prev() {
            let source_node = SourceNode::Next(i);
            validate_reference(veclen, index_of, source_node, prev);
        }
        if let Some(next) = node.next() {
            let source_node = SourceNode::Prev(i);
            validate_reference(veclen, index_of, source_node, next);
        }
    }
}
#[cfg(test)]
pub(crate) fn validate_list_x<'a, T, P>(list: &LinkedListX<'a, T, P>)
where
    T: 'a,
    P: PinnedVec<LinkedListNode<'a, T>> + 'a,
{
    use crate::linked_list::IS_SOME;

    let veclen = list.vec.len();
    let index_of = |x: &LinkedListNode<'a, T>| list.vec.index_of(x);
    let node = list.vec.get(0).expect(IS_SOME);
    if let Some(prev) = node.prev() {
        let source_node = SourceNode::Sentinel;
        validate_reference(veclen, index_of, source_node, prev);
    }
    if let Some(next) = node.next() {
        let source_node = SourceNode::Sentinel;
        validate_reference(veclen, index_of, source_node, next);
    }

    for i in 1..veclen {
        let node = list.vec.get(i).expect(IS_SOME);
        if let Some(prev) = node.prev() {
            let source_node = SourceNode::Next(i);
            validate_reference(veclen, index_of, source_node, prev);
        }
        if let Some(next) = node.next() {
            let source_node = SourceNode::Prev(i);
            validate_reference(veclen, index_of, source_node, next);
        }
    }
}

#[cfg(test)]
fn validate_reference<'a, T, I>(
    veclen: usize,
    index_of: I,
    source_node: SourceNode,
    target_node_ref: &'a LinkedListNode<'a, T>,
) where
    T: 'a,
    I: Fn(&LinkedListNode<'a, T>) -> Option<usize>,
{
    let ind_target = index_of(target_node_ref).expect("reference does not belong to the list");
    assert!(
        (1..veclen).contains(&ind_target),
        "reference index is out of bounds"
    );

    let (i, source_node_ref) = match source_node {
        SourceNode::Prev(i) => (
            i,
            target_node_ref
                .prev()
                .expect("node targeted as a next does not have a prev"),
        ),
        SourceNode::Next(i) => (
            i,
            target_node_ref
                .next()
                .expect("node targeted as a prev does not have a next"),
        ),
        SourceNode::Sentinel => {
            return;
        }
    };
    let source_index = index_of(source_node_ref).expect("node does not belong to the list");
    assert_eq!(i, source_index);
}
