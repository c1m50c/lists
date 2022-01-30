/// Shorthand syntax for creating a [`SinglyLinkedList`].
/// 
/// ## Example
/// ```rust
/// let list = sl_list![1, 2, 3, 4, 5];
/// 
/// assert_eq!(list.len(), 5);
/// assert_eq!(list.front(), Some(&1));
/// ```
#[allow(unused_macros)]
macro_rules! sl_list {
    ( $( $element: expr ), * ) => {
        {
            let mut list = $crate::linked::singly::SinglyLinkedList::new();
            $( list.push_back($element); ) *
            list
        }
    };
}


#[allow(unused_imports)]
pub(crate) use sl_list;