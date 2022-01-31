/// Shorthand syntax for creating a [`DoublyLinkedList`].
/// 
/// ## Example
/// ```rust
/// let list = dl_list![1, 2, 3, 4, 5];
/// 
/// assert_eq!(list.len(), 5);
/// assert_eq!(list.front(), Some(&1));
/// assert_eq!(list.back(), Some(&5));
/// ```
#[allow(unused_macros)]
macro_rules! dl_list {
    ( $( $element: expr ), * ) => {
        {
            let mut list = $crate::linked::doubly::DoublyLinkedList::new();
            $( list.push_back($element); ) *
            list
        }
    };
}


#[allow(unused_imports)]
pub(crate) use dl_list;