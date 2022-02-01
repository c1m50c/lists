//! Module containing data-structures that are dynamically allocated sequences, such as a [`List`].
//! Data-structures similar to these are the most common way to represent one-dimensional data now and days.
//! They are very good for optimization techniques such as cache optimization, and normally have `O(1)` lookup times.
//! Macros for shorthand construction of the various lists are availible within the library’s root.
//! 
//! ## Lists
//! ```rust
//! pub struct List<T> { .. } // Dynamically Allocated `List`.
//! ```


pub mod list;


pub use list::List;