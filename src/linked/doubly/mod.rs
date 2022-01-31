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
use core::ptr::{read as ptr_read, NonNull};
use core::iter::{Iterator, IntoIterator, DoubleEndedIterator, FusedIterator, ExactSizeIterator};
use core::cmp::{Eq, PartialEq};
use core::option::Option;
use core::fmt;


/// A two-directional linked list, known more commonly as a [`DoublyLinkedList`].
pub struct DoublyLinkedList<T> {
    /// [`Node`] at the `front` of the [`DoublyLinkedList`].
    head: Option<NonNull<Node<T>>>,

    /// [`Node`] at the `back` of the [`DoublyLinkedList`].
    tail: Option<NonNull<Node<T>>>,

    /// Length of the [`DoublyLinkedList`], represents how many [`Node`]s are contained within.
    len: usize,
}


/// [`Iter`] for a [`DoublyLinkedList`], it is the list's struct for their `IntoIter` trait.
pub struct Iter<T> {
    /// [`DoublyLinkedList`] used in iterating over.
    list: DoublyLinkedList<T>,
}


impl<T> Iterator for Iter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        return self.list.pop_front();
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        return ( self.list.len(), Some(self.list.len()) );
    }
}


impl<T> DoubleEndedIterator for Iter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        return self.list.pop_back();
    }
}


impl<T> FusedIterator for Iter<T> {  }
impl<T> ExactSizeIterator for Iter<T> {  }


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

    /// Returns a reference to the [`Node`] at the given `index`.
    /// Time complexity is `O(n)`.
    /// 
    /// ## Example
    /// ```rust
    /// let list = dl_list![1, 2, 3];
    /// 
    /// assert_eq!(list.get(1), Some(&2));
    /// ```
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index == 0 { return self.front(); }
        if index == self.len - 1 { return self.back(); }

        let mut current = self.head;
        let mut i = 0;

        while let Some(ptr) = current {
            let node = unsafe { ptr.as_ref() };

            if i == index {
                return Some(&node.value);
            }

            current = node.next;
            i += 1;
        }

        return None;
    }

    /// Returns a mutable reference to the [`Node`] at the given `index`.
    /// Time complexity is `O(n)`.
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = dl_list![1, 2, 3];
    /// 
    /// assert_eq!(list.get_mut(1), Some(&mut 2));
    /// ```
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index == 0 { return self.front_mut(); }
        if index == self.len - 1 { return self.back_mut(); }

        let mut current = self.head;
        let mut i = 0;

        while let Some(mut ptr) = current {
            let node = unsafe { ptr.as_mut() };

            if i == index {
                return Some(&mut node.value);
            }

            current = node.next;
            i += 1;
        }

        return None;
    }

    /// Removes the list's `head` [`Node`], returning its `value`.
    /// Time complexity is `O(1)`.
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = dl_list![1, 2, 3];
    /// let pop = list.pop_front();
    /// 
    /// assert_eq!(pop, Some(1));
    /// assert_eq!(list, dl_list![2, 3]);
    /// ```
    #[inline]
    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(mut ptr) = self.head {
            let value;

            unsafe {
                value = ptr_read(&mut (*ptr.as_mut()).value);
                self.head = (*self.head.unwrap().as_ptr()).next;

                if let Some(ptr) = self.head {
                    (*ptr.as_ptr()).prev = None;
                }
            }

            self.len -= 1;
            return Some(value);
        }

        return None;
    }

    /// Removes the list's `tail` [`Node`], returning its `value`.
    /// Time complexity is `O(1)`.
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = dl_list![1, 2, 3];
    /// let pop = list.pop_back();
    /// 
    /// assert_eq!(pop, Some(3));
    /// assert_eq!(list, dl_list![1, 2]);
    /// ```
    #[inline]
    pub fn pop_back(&mut self) -> Option<T> {
        if let Some(mut ptr) = self.tail {
            let value;

            unsafe {
                value = ptr_read(&mut (*ptr.as_mut()).value);
                self.tail = (*self.tail.unwrap().as_ptr()).prev;

                if let Some(ptr) = self.tail {
                    (*ptr.as_ptr()).next = None;
                }
            }

            self.len -= 1;
            return Some(value);
        }

        return None;
    }

    /// Removes the list's `head` [`Node`].
    /// Time complexity is `O(1)`.
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = dl_list![1, 2, 3];
    /// list.remove_front();
    /// 
    /// assert_eq!(list, dl_list![2, 3]);
    /// ```
    #[inline]
    pub fn remove_front(&mut self) {
        let _ = self.pop_front();
    }

    /// Removes the list's `tail` [`Node`].
    /// Time complexity is `O(1)`.
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = dl_list![1, 2, 3];
    /// list.remove_back();
    /// 
    /// assert_eq!(list, dl_list![1, 2]);
    /// ```
    #[inline]
    pub fn remove_back(&mut self) {
        let _ = self.pop_back();
    }
}


impl<T: PartialEq> PartialEq for DoublyLinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len { return false; }
        if self.len == 0 { return true; }

        let mut s = self.head;
        let mut o = other.head;

        while let (Some(a), Some(b)) = (s, o) {
            let a = unsafe { a.as_ref() };
            let b = unsafe { b.as_ref() };

            if a.value != b.value { return false; }
            s = a.next; o = b.next;
        }

        return true;
    }
}


impl<T: Eq> Eq for DoublyLinkedList<T> {  }


impl<T: fmt::Debug> fmt::Debug for DoublyLinkedList<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return f.debug_struct("DoublyLinkedList")
            .field("head", &self.head)
            .field("tail", &self.tail)
            .field("len", &self.len)
            .finish();
    }
}


impl<T> IntoIterator for DoublyLinkedList<T> {
    type Item = T;
    type IntoIter = Iter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        return Iter { list: self };
    }
}