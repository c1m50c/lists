use super::{SinglyLinkedList, macros::sl_list};


#[test]
fn new() {
    let list = SinglyLinkedList::<()>::new();

    assert_eq!(list.len(), 0);
    assert_eq!(list.front(), None);
}


#[test]
fn front() {
    let mut list = SinglyLinkedList::new();
    
    list.push_front(6);
    list.push_front(7);
    list.push_front(0);

    assert_eq!(list.front_mut(), Some(&mut 0));
    assert_eq!(list.front(), Some(&0));
}


#[test]
fn push_front() {
    let mut list = SinglyLinkedList::new();

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    assert_eq!(list.len(), 3);
    assert_eq!(list.front(), Some(&3));
}


#[test]
fn push_back() {
    let mut list = SinglyLinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert_eq!(list.len(), 3);
    assert_eq!(list.front(), Some(&1));
}


#[test]
fn sl_list_macro() {
    let list = sl_list![1, 2, 3, 4, 5];
    assert_eq!(list.len(), 5);
    assert_eq!(list.front(), Some(&1));
}