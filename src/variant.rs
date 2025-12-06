use crate::memory::{DoublyReclaimer, SinglyReclaimer};
use core::marker::PhantomData;
use orx_selfref_col::{MemoryReclaimer, RefsArray, RefsNone, RefsSingle, Variant};

pub trait ListVariant: Variant {
    type Reclaimer: MemoryReclaimer<Self>;
}

/// A self referential collection variant representing a singly linked list
/// where nodes hold a reference to the next element, but not to the previous.
pub struct Singly<T> {
    p: PhantomData<T>,
}

/// # SAFETY
///
/// List variants do not hold any data, safe to send or sync.
unsafe impl<T> Sync for Singly<T> {}

impl<T> Variant for Singly<T> {
    type Item = T;

    type Prev = RefsNone;

    type Next = RefsSingle<Self>;

    type Ends = RefsSingle<Self>;
}

impl<T> ListVariant for Singly<T> {
    type Reclaimer = SinglyReclaimer;
}

/// A self referential collection variant representing a doubly linked list
/// where nodes hold a reference to the next element, and a reference to the previous.
pub struct Doubly<T> {
    p: PhantomData<T>,
}

/// # SAFETY
///
/// List variants do not hold any data, safe to send or sync.
unsafe impl<T> Sync for Doubly<T> {}

impl<T> Variant for Doubly<T> {
    type Item = T;

    type Prev = RefsSingle<Self>;

    type Next = RefsSingle<Self>;

    type Ends = RefsArray<2, Self>;
}

impl<T> ListVariant for Doubly<T> {
    type Reclaimer = DoublyReclaimer;
}
