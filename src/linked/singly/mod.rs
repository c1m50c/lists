#[cfg(test)]
mod tests;

mod node;

use std::boxed::Box;
use core::option::Option;
use core::ptr::NonNull;
use node::Node;


/// A one-directional linked list, known more commonly as a [`SinglyLinkedList`].
pub struct SinglyLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    len: usize,
}


impl<T> SinglyLinkedList<T> {
    #[inline]
    pub const fn new() -> Self {
        return Self {
            head: None,
            len: 0,
        };
    }

    /// Returns the `len` or the number of [`Node`]s within the [`SinglyLinkedList`].
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = SinglyLinkedList::new();
    /// 
    /// list.push_front(3);
    /// list.push_front(2);
    /// list.push_front(1);
    /// 
    /// assert_eq!(list.len(), 3);
    /// ```
    #[inline]
    pub const fn len(&self) -> usize {
        return self.len;
    }

    /// Clears the [`SinglyLinkedList`] settings its fields back to the default values.
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = SinglyLinkedList::new();
    /// 
    /// list.push_front(4);
    /// list.push_front(0);
    /// list.push_front(4);
    /// assert_eq!(list.len(), 3);
    /// 
    /// list.clear()
    /// assert_eq!(list.len(), 0)
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    /// Returns a reference to the [`Node`] at the `front` of the [`SinglyLinkedList`], also known as the `head`.
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = SinglyLinkedList::new();
    /// 
    /// list.push_front(1);
    /// list.push_front(2);
    /// 
    /// assert_eq!(list.front(), Some(&2));
    /// ```
    #[inline]
    pub fn front(&self) -> Option<&T> {
        return match self.head {
            Some(ptr) => unsafe { Some(&ptr.as_ref().value) },
            None => None,
        };
    }

    /// Returns a mutable reference to the [`Node`] at the `front` of the [`SinglyLinkedList`], also known as the `head`.
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = SinglyLinkedList::new();
    /// 
    /// list.push_front(1);
    /// list.push_front(2);
    /// 
    /// assert_eq!(list.front(), Some(&mut 2));
    /// ```
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        return match self.head {
            Some(mut ptr) => unsafe { Some(&mut ptr.as_mut().value) },
            None => None,
        };
    }

    /// Pushes a new [`Node`] with the coresponding `value` to the `front` of the list, making the list's `head` the new [`Node`].
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = SinglyLinkedList::new();
    /// 
    /// list.push_front(4);
    /// list.push_front(5);
    /// list.push_front(6);
    /// 
    /// assert_eq!(list.front(), Some(&6));
    /// ```
    #[inline]
    pub fn push_front(&mut self, value: T) {
        let mut new_node = Node::new(value).into_box();
        new_node.next = self.head;

        let ptr = unsafe {
            NonNull::new_unchecked(Box::into_raw(new_node))
        };

        self.len += 1;
        self.head = Some(ptr);
    }
}