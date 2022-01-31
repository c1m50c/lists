//! # Linked
//! Module containing data-structures that resemble `LinkedList`s.
//! `LinkedList`s are widely unused in modern computing due the `Vector` data structure being more superior in just about every aspect now and days.
//! `Vector`s are much more cache-optimized that `LinkedList`s and their lookup times resemble `O(1)` time complexity, making them better for most applications.
//! Macros for shorthand construction of the various lists are availible within the library's root.
//! 
//! ## Lists
//! ```rust
//! pub struct SinglyLinkedList<T> { .. } // One-directional `LinkedList`.
//! pub struct DoublyLinkedList<T> { .. } // Two-directional `LinkedList`.
//! ```


pub(crate) mod singly;
pub(crate) mod doubly;


pub use singly::SinglyLinkedList;
pub use doubly::DoublyLinkedList;