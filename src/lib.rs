//! Library containing various implementations of list-like data-structures such as `Vector`s, `LinkedList`s, and more.
//! All data-structures follow a sequence-like structure and can be represented like an `Array`.
//! 
//! ## Lists
//! ```rust
//! pub struct SinglyLinkedList<T> { .. } // One-directional `LinkedList`.
//! pub struct DoublyLinkedList<T> { .. } // Two-directional `LinkedList`.
//! ```



pub mod linked;


pub use linked::singly::SinglyLinkedList;
pub use linked::doubly::DoublyLinkedList;


/// Shorthand syntax for creating a [`SinglyLinkedList`].
/// Time complexity is `O(n)`.
/// 
/// ## Example
/// ```rust
/// let list = sl_list![1, 2, 3, 4, 5];
/// 
/// assert_eq!(list.len(), 5);
/// assert_eq!(list.front(), Some(&1));
/// assert_eq!(list.back(), Some(&5));
/// ```
#[macro_export]
macro_rules! sl_list {
    ( $( $element: expr ), * ) => {
        {
            let mut list = $crate::linked::singly::SinglyLinkedList::new();
            $( list.push_back($element); ) *
            list
        }
    };
}


/// Shorthand syntax for creating a [`DoublyLinkedList`].
/// Time complexity is `O(1)`.
/// 
/// ## Example
/// ```rust
/// let list = dl_list![1, 2, 3, 4, 5];
/// 
/// assert_eq!(list.len(), 5);
/// assert_eq!(list.front(), Some(&1));
/// assert_eq!(list.back(), Some(&5));
/// ```
#[macro_export]
macro_rules! dl_list {
    ( $( $element: expr ), * ) => {
        {
            let mut list = $crate::linked::doubly::DoublyLinkedList::new();
            $( list.push_back($element); ) *
            list
        }
    };
}