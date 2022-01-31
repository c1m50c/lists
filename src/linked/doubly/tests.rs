use super::super::super::dl_list;
use super::DoublyLinkedList;


#[test]
fn new() {
    let list = DoublyLinkedList::<()>::new();

    assert_eq!(list.len(), 0);
    assert_eq!(list.front(), None);
    assert_eq!(list.back(), None);
}


#[test]
fn front_back() {
    let mut list = dl_list![1, 2];

    assert_eq!(list.front_mut(), Some(&mut 1));
    assert_eq!(list.back_mut(), Some(&mut 2));
    assert_eq!(list.front(), Some(&1));
    assert_eq!(list.back(), Some(&2));
}


#[test]
fn push() {
    let mut list = DoublyLinkedList::new();

    list.push_front(1);
    list.push_front(2);

    assert_eq!(list.len(), 2);
    assert_eq!(list.front(), Some(&2));
    assert_eq!(list.back(), Some(&1));

    let mut list = DoublyLinkedList::new();

    list.push_back(1);
    list.push_back(2);

    assert_eq!(list.len(), 2);
    assert_eq!(list.front(), Some(&1));
    assert_eq!(list.back(), Some(&2));
}


#[test]
fn get() {
    let mut list = dl_list![1, 2, 3];

    assert_eq!(list.get_mut(0), Some(&mut 1));
    assert_eq!(list.get_mut(1), Some(&mut 2));
    assert_eq!(list.get_mut(2), Some(&mut 3));
    assert_eq!(list.get(0), Some(&1));
    assert_eq!(list.get(1), Some(&2));
    assert_eq!(list.get(2), Some(&3));
}