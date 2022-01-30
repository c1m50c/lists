use std::boxed::Box;
use core::option::Option;
use core::cmp::PartialEq;
use core::ptr::NonNull;
use core::fmt;


/// [`Node`] for representing values in a [`SinglyLinkedList`].
pub struct Node<T> {
    pub next: Option<NonNull<Node<T>>>,
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
}


impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}


impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return f.debug_struct("Node")
            .field("next", &self.next)
            .field("value", &self.value)
            .finish();
    }
}