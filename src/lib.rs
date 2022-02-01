//! Library containing various implementations of list-like data-structures such as `List`s, `LinkedList`s, and more.
//! All data-structures follow a sequence-like structure and can be represented as such.
//! 
//! ## Lists
//! ```rust
//! pub struct SinglyLinkedList<T> { .. } // One-directional `LinkedList`.
//! pub struct DoublyLinkedList<T> { .. } // Two-directional `LinkedList`.
//! pub struct List<T> { .. } // Dynamically Allocated `List`.
//! ```


pub mod linked;
pub mod dynamic;


pub use linked::singly::SinglyLinkedList;
pub use linked::doubly::DoublyLinkedList;
pub use dynamic::list::List;


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


/// Shorthand syntax for creating a [`List`].
/// Time complexity is `O(1)`.
/// 
/// ## Example
/// ```rust
/// let list = list![1, 2, 3, 4, 5];
/// 
/// assert_eq!(list.len(), 5);
/// assert_eq!(list[0], 1);
/// assert_eq!(list[4], 5);
/// ```
#[macro_export]
macro_rules! list {
    ( $( $element: expr ), * ) => {
        {
            let mut list = $crate::dynamic::list::List::new();
            $( list.push($element); ) *
            list
        }
    };
}