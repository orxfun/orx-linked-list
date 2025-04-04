use crate::Singly;
use orx_selfref_col::{Node, NodePtr};

/// A node pointer in a Singly linked list.
pub type SinglyPtr<T> = NodePtr<Singly<T>>;

impl<T> SinglyPointer<T> for SinglyPtr<T> {
    #[inline(always)]
    fn raw_ptr(&self) -> *mut Node<Singly<T>> {
        self.ptr() as *mut Node<Singly<T>>
    }
}

/// A node pointer in a Singly linked list.
pub trait SinglyPointer<T> {
    /// Returns the raw pointer to the node.
    fn raw_ptr(&self) -> *mut Node<Singly<T>>;

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
    unsafe fn node(&self) -> &Node<Singly<T>> {
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
    unsafe fn node_mut(&mut self) -> &mut Node<Singly<T>> {
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
    unsafe fn next(&self) -> Option<SinglyPtr<T>> {
        unsafe { self.node() }.next().get().cloned()
    }
}
