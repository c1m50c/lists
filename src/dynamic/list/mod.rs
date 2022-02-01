//! Module containing a [`List`] data-structure.
//! A [`List`] is a one-dimensional sequence data-structure that is dynamically allocated to hold a virtually infinite number of items.
//! They benefit from various factors such as `O(1)` lookup times and cache optimization.
//! 
//! ## Lists
//! ```rust
//! pub struct List<T> { .. } // Dynamically Allocated `List`.
//! ```


#[cfg(test)]
mod tests;


use core::slice::from_raw_parts_mut;
use core::mem::{size_of, align_of};
use core::ops::{Index, IndexMut};
use core::ptr::drop_in_place;
use core::option::Option;
use core::ptr::NonNull;
use std::alloc;


/// The `capacity` will be multiplied by this whenever reallocation is needed.
pub const RESIZE_MULTIPLIER: usize = 2;

/// The initial `capacity` when the first `push()` is called for the [`List`].
pub const INITIAL_CAPACITY: usize = 4;


/// A one-dimensional, dynamically allocated sequence, known more commonly as a [`List`].
pub struct List<T> {
    /// `ptr` to the first item within the [`List`].
    ptr: NonNull<T>,

    /// The `capacity` of the [`List`] represents how many items it can hold without reallocating memory.
    capacity: usize,
    
    /// The `len` of the [`List`] represents how many items are present within.
    len: usize,
}


impl<T> List<T> {
    /// Creates a new, and empty [`List`].
    #[inline]
    pub const fn new() -> Self {
        return Self {
            ptr: NonNull::dangling(),
            capacity: 0,
            len : 0,
        }
    }

    /// Returns the `capacity` field of the [`List`].
    #[inline]
    pub const fn capacity(&self) -> usize {
        return self.capacity;
    }

    /// Returns the `len` field of the [`List`].
    #[inline]
    pub const fn len(&self) -> usize {
        return self.len;
    }

    /// Appends a new `value` into the [`List`].
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = List::new();
    /// 
    /// list.push(1);
    /// list.push(2);
    /// list.push(3);
    /// 
    /// assert_eq!(list, list![1, 2, 3]);
    /// ```
    pub fn push(&mut self, value: T) {
        assert!(size_of::<T>() > 0, "Zero-sized types are not allowed.");
        
        // Initialize allocation
        if self.capacity == 0 {
            let layout = alloc::Layout::array::<T>(INITIAL_CAPACITY)
                .expect("Could not allocate memory."); // TODO: More verbose panic message.
            
            // SAFETY: Layout is hardcoded to be `INITIAL_CAPACITY * size_of<T>`, and `size_of<T> > 0`.
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;

            let ptr = NonNull::new(ptr)
                .expect("Could not allocate memory."); // TODO: More verbose panic message.

            // SAFETY: `ptr` is non-null and we've allocated enough space for the value.
            unsafe { ptr.as_ptr().write(value); }
            
            self.ptr = ptr;
            self.capacity = INITIAL_CAPACITY;
            self.len = 1;
        }

        // Push value into allocation
        else if self.len < self.capacity {
            let offset = self.len
                .checked_mul(size_of::<T>())
                .expect("Cannot reach memory location."); // TODO: More verbose panic message.
            
            assert!(offset < isize::MAX as usize, "Wrapped `isize`.");

            /*
                SAFETY:
                - Offset cannot wrap around
                - `ptr` is pointing to valid memory
                - Writing to an offset at `len` is valid
            */
            unsafe { self.ptr.as_ptr().add(self.len).write(value); }

            self.len += 1;
        }

        // Reallocate & push
        else {
            debug_assert!(self.len == self.capacity);

            let new_capacity = self.capacity.checked_mul(RESIZE_MULTIPLIER)
                .expect("Capacity wrapped."); // TODO: More verbose panic message.
            
            let new_size = size_of::<T>() * new_capacity;
            let size = size_of::<T>() * self.capacity;
            let align = align_of::<T>();

            size.checked_add(size % align)
                .expect("Cannot allocate memory."); // TODO: More verbose panic message.
    
            unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);

                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);

                let ptr = NonNull::new(ptr as *mut T)
                    .expect("Could not reallocate."); // TODO: More verbose panic message.
                
                ptr.as_ptr().add(self.len).write(value);
                self.ptr = ptr;
            }

            self.len += 1;
            self.capacity = new_capacity;
        }
    }

    /// Returns a reference to the item at the given `index`.
    /// 
    /// ## Example
    /// ```rust
    /// let list = list![1, 2, 3];
    /// 
    /// assert_eq!(list.get(0), Some(&1));
    /// ```
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len { return None; }

        return unsafe {
            Some(&*self.ptr.as_ptr().add(index))
        };
    }

    /// Returns a mutable reference to the item at the given `index`.
    /// 
    /// ## Example
    /// ```rust
    /// let mut list = list![1, 2, 3];
    ///
    /// *list.get_mut(0).unwrap() = 4;
    /// assert_eq!(list.get(0), Some(&4));
    /// ```
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len { return None; }

        return unsafe {
            Some(&mut *self.ptr.as_ptr().add(index))
        };
    }
}


impl<T> Drop for List<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            drop_in_place(
                from_raw_parts_mut(self.ptr.as_ptr(), self.len)
            );

            let layout = alloc::Layout::from_size_align_unchecked(
                size_of::<T>() * self.capacity,
                align_of::<T>()
            );

            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}


impl<T> Index<usize> for List<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        return self.get(index)
            .expect(format!("Index '{}' out of bounds.", index).as_str());
    }
}


impl<T> IndexMut<usize> for List<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.get_mut(index)
            .expect(format!("Index '{}' out of bounds.", index).as_str());
    }
}