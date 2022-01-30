pub(crate) mod macros;
mod node;

#[cfg(test)]
mod tests;

use node::Node;
use std::boxed::Box;
use core::ptr::{NonNull, read as ptr_read};
use core::iter::{Iterator, IntoIterator, ExactSizeIterator};
use core::cmp::{Eq, PartialEq};
use core::option::Option;
use core::fmt;


/// A one-directional linked list, known more commonly as a [`SinglyLinkedList`].
pub struct SinglyLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    len: usize,
}


/// Version of a [`SinglyLinkedList`] that implements the [`Iterator`] trait, a [`SinglyLinkedList`]'s [`IntoIter`].
pub struct Iter<T> {
    list: SinglyLinkedList<T>,
}


impl<T> Iterator for Iter<T> {
    type Item = T;
    
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        return self.list.pop_front();
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        return (self.list.len(), Some(self.list.len()));
    }
}


impl<T> ExactSizeIterator for Iter<T> {  }


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

    /// Pushes a new [`Node`] with the coresponding `value` to the `back` of the list, making the list's last [`Node`] the new [`Node`].
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = SinglyLinkedList::new();
    /// 
    /// list.push_back(4);
    /// list.push_back(5);
    /// list.push_back(6);
    /// 
    /// assert_eq!(list.front(), Some(&4));
    /// ```
    #[inline]
    pub fn push_back(&mut self, value: T) {
        let ptr = unsafe {
            NonNull::new_unchecked(
                Box::into_raw(Node::new(value).into_box())
            )
        };

        match self.head {
            Some(x) => unsafe {
                let mut current = Some(x);

                while let Some(mut x) = current {
                    let m = x.as_mut();

                    if m.next.is_none() {
                        m.next = Some(ptr);
                        break;
                    }

                    current = m.next;
                }
            },

            None => self.head = Some(ptr),
        }

        self.len += 1
    }

    /// Removes the [`Node`] at the `front` of the list and returns its `value` field.
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = sl_list![3, 0, 0, 5];
    /// let value = list.pop_front();
    /// 
    /// assert_eq!(value, Some(3));
    /// assert_eq!(list, sl_list![0, 0, 5]);
    /// ```
    #[inline]
    pub fn pop_front(&mut self) -> Option<T> {
        return match self.head {
            Some(mut ptr) => unsafe {
                let node = ptr_read(&mut (*ptr.as_mut()));
                self.head = node.next;
                self.len -= 1;
                Some(node.value)
            },

            None => None,
        };
    }
}


impl<T> IntoIterator for SinglyLinkedList<T> {
    type Item = T;
    type IntoIter = Iter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        return Iter { list: self };
    }
}


impl<T: PartialEq> PartialEq for SinglyLinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() { return false; }
        if self.len() == 0 { return true; }

        let mut s = self.head;
        let mut o = other.head;

        while let (Some(a), Some(b)) = (s, o) {
            let a = unsafe { a.as_ref() };
            let b = unsafe { b.as_ref() };

            if a.value != b.value { return false; }

            s = a.next;
            o = b.next;
        }
        
        return true;
    }
}


impl<T: Eq> Eq for SinglyLinkedList<T> {  }


impl<T: fmt::Debug> fmt::Debug for SinglyLinkedList<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return f.debug_struct("SinglyLinkedList")
            .field("head", &self.head)
            .field("len", &self.len)
            .finish();
    }
}