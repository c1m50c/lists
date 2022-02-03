//! Contains the [`SinglyLinkedList`]'s [`Node`], the [`Node`] contains the `value`s & `ptr`s of all elements within the list.


use std::boxed::Box;
use core::option::Option;
use core::cmp::PartialEq;
use core::ptr::NonNull;
use core::fmt;


/// [`Node`] for representing values in a [`SinglyLinkedList`].
pub struct Node<T> {
    /// Next [`Node`] within the [`SinglyLinkedList`].
    pub next: Option<NonNull<Node<T>>>,

    /// Value of the [`Node`].
    pub value: T,
}


impl<T> Node<T> {
    /// Creates a new [`Node`] with the coresponding `value` and a `None`in the `next` field.
    /// 
    /// ## Example
    /// ```rust
    /// let node = Node::new("New Node");
    /// assert_eq!(node.next, None);
    /// assert_eq!(node.value, "New Node");
    /// ```
    #[inline]
    pub const fn new(value: T) -> Self {
        return Self {
            next: None,
            value,
        };
    }

    /// Converts the [`Node`] into a [`Box`]-ed version of the [`Node`].
    /// 
    /// ## Example
    /// ```rust
    /// let boxed_node = Node::new(5).into_box();
    /// assert_eq!(boxed_node, Box::new(Node::new(5)));
    /// ```
    #[inline]
    pub fn into_box(self) -> Box<Self> {
        return Box::new(self);
    }

    /// Converts the [`Node`] into a [`Box`], then converts the [`Box`] into a [`NonNull`] of the [`Node`].
    /// 
    /// ## Example
    /// ```rust
    /// let ptr = Node::new(5).into_non_null();
    /// let value = unsafe { &ptr.as_ref().value };
    /// 
    /// assert_eq!(value, &5);
    /// ```
    /// 
    /// ## Safety
    /// - Converts the [`Node`] into a [`Box`] before [`NonNull`] conversion, `ptr` should always be valid.
    #[inline]
    pub fn into_non_null(self) -> NonNull<Self> {
        return unsafe { NonNull::new_unchecked(Box::into_raw(self.into_box())) };
    }
}


impl<T: PartialEq> PartialEq for Node<T> {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        return self.value == rhs.value;
    }
}


impl<T: PartialEq> PartialEq<T> for Node<T> {
    #[inline]
    fn eq(&self, rhs: &T) -> bool {
        return self.value == *rhs;
    }
}


impl<T: fmt::Debug> fmt::Debug for Node<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return f.debug_struct("Node")
            .field("next", &self.next)
            .field("value", &self.value)
            .finish();
    }
}