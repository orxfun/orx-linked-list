use crate::Doubly;
use orx_selfref_col::{Node, NodePtr};

/// A node pointer in a doubly linked list.
pub type DoublyPtr<T> = NodePtr<Doubly<T>>;

impl<T> DoublyPointer<T> for DoublyPtr<T> {
    #[inline(always)]
    unsafe fn raw_ptr(&self) -> *mut Node<Doubly<T>> {
        unsafe { self.ptr() as *mut Node<Doubly<T>> }
    }
}

/// A node pointer in a doubly linked list.
pub trait DoublyPointer<T> {
    /// Returns the raw pointer to the node.
    ///
    /// # SAFETY
    ///
    /// This method is unsafe as node pointers implement `Send` and `Sync`.
    ///
    /// It is safe dereference the received pointer if we know that `is_valid_for(col)` would
    /// return `true` where `col` is the collection that this pointer is created from.
    unsafe fn raw_ptr(&self) -> *mut Node<Doubly<T>>;

    /// Returns a reference to the node.
    ///
    /// # Safety
    ///
    /// This method creates a reference directly from
    /// the pointer without any checks.
    ///
    /// The caller is responsible for the validness
    /// of the node pointer.
    ///
    /// Alternatively, you may use `NodeIdx` for safe access.
    #[inline(always)]
    unsafe fn node(&self) -> &Node<Doubly<T>> {
        unsafe { &*self.raw_ptr() }
    }

    /// Returns a mutable reference to the node.
    ///
    /// # Safety
    ///
    /// This method creates a reference directly from
    /// the pointer without any checks.
    ///
    /// The caller is responsible for the validness
    /// of the node pointer.
    ///
    /// Alternatively, you may use `NodeIdx` for safe access.
    #[inline(always)]
    unsafe fn node_mut(&mut self) -> &mut Node<Doubly<T>> {
        unsafe { &mut *self.raw_ptr() }
    }

    /// Returns the pointer to the next node if exists; None otherwise.
    ///
    /// # Safety
    ///
    /// This method creates a reference directly from
    /// the pointer without any checks.
    ///
    /// The caller is responsible for the validness
    /// of the node pointer.
    ///
    /// Alternatively, you may use `NodeIdx` for safe access.
    #[inline(always)]
    unsafe fn next(&self) -> Option<DoublyPtr<T>> {
        unsafe { self.node() }.next().get()
    }

    /// Returns the pointer to the prev node if exists; None otherwise.
    ///
    /// # Safety
    ///
    /// This method creates a reference directly from
    /// the pointer without any checks.
    ///
    /// The caller is responsible for the validness
    /// of the node pointer.
    ///
    /// Alternatively, you may use `NodeIdx` for safe access.
    #[inline(always)]
    unsafe fn prev(&self) -> Option<DoublyPtr<T>> {
        unsafe { self.node() }.prev().get()
    }
}
