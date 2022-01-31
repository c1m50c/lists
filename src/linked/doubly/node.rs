use std::boxed::Box;
use core::ptr::NonNull;
use core::option::Option;
use core::fmt;


/// [`Node`] for representing values in a [`DoublyLinkedList`].
pub struct Node<T> {
    pub next: Option<NonNull<Node<T>>>,
    pub prev: Option<NonNull<Node<T>>>,
    pub value: T,
}


impl<T> Node<T> {
    /// Creates a new [`Node`] with the coresponding `value` and a `None`in the `next` & `prev` fields.
    /// 
    /// ## Example
    /// ```rust
    /// let node = Node::new("New Node");
    /// assert_eq!(node.next, None);
    /// assert_eq!(node.prev, None);
    /// assert_eq!(node.value, "New Node");
    /// ```
    pub const fn new(value: T) -> Self {
        return Self {
            next: None,
            prev: None,
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
            .field("prev", &self.prev)
            .field("value", &self.value)
            .finish();
    }
}