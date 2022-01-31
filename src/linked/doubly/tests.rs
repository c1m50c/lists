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


#[test]
fn pop() {
    let mut list = dl_list![1, 2, 3];
    let front = list.pop_front();
    let back = list.pop_back();

    assert_eq!(front, Some(1));
    assert_eq!(back, Some(3));
    assert_eq!(list, dl_list![2]);
}


#[test]
fn remove() {
    let mut list = dl_list![1, 2, 3];
    list.remove_front();
    list.remove_back();

    assert_eq!(list, dl_list![2]);
}


#[test]
fn eq() {
    let list_a = dl_list![4, 0, 0, 5];
    let list_b = dl_list![4, 0, 0, 5];
    
    assert_eq!(list_a, list_b);
}


#[test]
fn iter() {
    let list = dl_list![5, 5, 5, 5];
    assert_eq!(list.into_iter().sum::<i32>(), 20);
}