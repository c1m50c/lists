//! Module containing a [`DoublyLinkedList`] data-structure.
//! A [`DoublyLinkedList`] is a two-directional data-structure that contains [`Node`]s that point to the next & previous [`Node`]s within the sequence.
//! They contain [`Node`]s that have a `next`, `prev` and `value` field;
//! `next` points to the next [`Node`] in the sequence,
//! `prev` points to the previous [`Node`] in the sequence,
//! `value` holds the [`Node`]'s value, or data held within.
//! 
//! ## Lists
//! ```rust
//! pub struct DoublyLinkedList<T> { .. } // Two-directional `LinkedList`.
//! ```


#[cfg(test)]
mod tests;

mod node;

use node::Node;
use std::boxed::Box;
use core::option::Option;
use core::ptr::NonNull;


/// A two-directional linked list, known more commonly as a [`DoublyLinkedList`].
pub struct DoublyLinkedList<T> {
    /// [`Node`] at the `front` of the [`DoublyLinkedList`].
    head: Option<NonNull<Node<T>>>,

    /// [`Node`] at the `back` of the [`DoublyLinkedList`].
    tail: Option<NonNull<Node<T>>>,

    /// Length of the [`DoublyLinkedList`], represents how many [`Node`]s are contained within.
    len: usize,
}


impl<T> DoublyLinkedList<T> {
    /// Constructs a new, empty, [`DoublyLinkedList`].
    #[inline]
    pub const fn new() -> Self {
        return Self {
            head: None,
            tail: None,
            len: 0,
        };
    }

    /// Returns the `len` or the number of [`Node`]s within the [`DoublyLinkedList`].
    /// 
    /// ## Example
    /// ```rust
    /// let list = dl_list![1, 2, 3];
    /// 
    /// assert_eq!(list.len(), 3);
    /// ```
    #[inline]
    pub const fn len(&self) -> usize {
        return self.len;
    }

    /// Clears the [`DoublyLinkedList`] settings its fields back to their default values.
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = dl_list![1, 2, 3, 4, 5];
    /// list.clear();
    /// 
    /// assert_eq!(list, DoublyLinkedList::<i32>::new());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    /// Returns a reference to the [`Node`] at the `front` of the [`DoublyLinkedList`], also known as the `head`.
    /// Time complexity is `O(1)`.
    /// 
    /// ## Example
    /// ```rust
    /// let list = dl_list![2, 4, 0];
    /// 
    /// assert_eq!(list.front(), Some(&2));
    /// ```
    #[inline]
    pub fn front(&self) -> Option<&T> {
        return match self.head {
            Some(ptr) => unsafe { Some(&ptr.as_ref().value) },
            None => None,
        }
    }

    /// Returns a reference to the [`Node`] at the `back` of the [`DoublyLinkedList`], also known as the `tail`.
    /// Time complexity is `O(1)`.
    /// 
    /// ## Example
    /// ```rust
    /// let list = dl_list![2, 4, 0];
    /// 
    /// assert_eq!(list.back(), Some(&0));
    /// ```
    #[inline]
    pub fn back(&self) -> Option<&T> {
        return match self.tail {
            Some(ptr) => unsafe { Some(&ptr.as_ref().value) },
            None => None,
        }
    }

    /// Returns a mutable reference to the [`Node`] at the `front` of the [`DoublyLinkedList`], also known as the `head`.
    /// Time complexity is `O(1)`.
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = dl_list![2, 4, 0];
    /// 
    /// assert_eq!(list.front_mut(), Some(&mut 2));
    /// ```
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        return match self.head {
            Some(mut ptr) => unsafe { Some(&mut ptr.as_mut().value) },
            None => None,
        }
    }

    /// Returns a mutable reference to the [`Node`] at the `back` of the [`DoublyLinkedList`], also known as the `tail`.
    /// Time complexity is `O(1)`.
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = dl_list![2, 4, 0];
    /// 
    /// assert_eq!(list.back_mut(), Some(&mut 0));
    /// ```
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        return match self.tail {
            Some(mut ptr) => unsafe { Some(&mut ptr.as_mut().value) },
            None => None,
        }
    }

    /// Pushes or prepends a new [`Node`] to the `front` of the [`DoublyLinkedList`].
    /// Time complexity is `O(1)`.
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = DoublyLinkedList::new();
    /// 
    /// list.push_front(1);
    /// list.push_front(2);
    /// list.push_front(3);
    /// 
    /// assert_eq!(list, dl_list![3, 2, 1]);
    /// ```
    #[inline]
    pub fn push_front(&mut self, value: T) {
        let mut new_node = Node::new(value).into_box();
        new_node.next = self.head;
        
        let node_ptr = unsafe {
            Some(NonNull::new_unchecked(Box::into_raw(new_node)))
        };

        match self.head {
            Some(mut ptr) => unsafe { ptr.as_mut().prev = node_ptr; },
            None => { self.tail = node_ptr; },
        }

        self.len += 1;
        self.head = node_ptr;
    }

    /// Pushes or appends a new [`Node`] to the `back` of the [`DoublyLinkedList`].
    /// Time complexity is `O(1)`.
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = DoublyLinkedList::new();
    /// 
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    /// 
    /// assert_eq!(list, dl_list![1, 2, 3]);
    /// ```
    #[inline]
    pub fn push_back(&mut self, value: T) {
        let mut new_node = Node::new(value).into_box();
        new_node.prev = self.tail;
        
        let node_ptr = unsafe {
            Some(NonNull::new_unchecked(Box::into_raw(new_node)))
        };

        match self.tail {
            Some(mut ptr) => unsafe { ptr.as_mut().next = node_ptr; },
            None => { self.head = node_ptr; },
        }

        self.len += 1;
        self.tail = node_ptr;
    }
}