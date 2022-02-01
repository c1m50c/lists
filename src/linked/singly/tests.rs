use super::super::super::sl_list;
use super::SinglyLinkedList;
use super::node::Node;


#[test]
fn new() {
    let list = SinglyLinkedList::<()>::new();

    assert_eq!(list.len(), 0);
    assert_eq!(list.front(), None);
}


#[test]
fn front() {
    let mut list = sl_list![6, 7, 0];
    
    assert_eq!(list.front_mut(), Some(&mut 6));
    assert_eq!(list.front(), Some(&6));
}


#[test]
fn back() {
    let mut list = sl_list![6, 7, 0];
    
    assert_eq!(list.back_mut(), Some(&mut 0));
    assert_eq!(list.back(), Some(&0));
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


#[test]
fn pop_front() {
    let mut list = sl_list![3, 0, 0, 5];
    let pop = list.pop_front();
    
    assert_eq!(pop, Some(3));
    assert_eq!(list, sl_list![0, 0, 5]);
}


#[test]
fn iter() {
    let list = sl_list![0, 1, 2, 3, 4];
    let mut sum = 0;

    for i in list {
        sum += i;
    }

    assert_eq!(sum, 10);
}


#[test]
fn eq() {
    let list_a = sl_list![1, 2, 3, 4, 5];
    let list_b = sl_list![1, 2, 3, 4, 5];

    assert_eq!(list_a, list_b);
}


#[test]
fn remove_front() {
    let mut list = sl_list![1, 2, 3];
    list.remove_front();

    assert_eq!(list, sl_list![2, 3]);
}


#[test]
fn get() {
    let mut list = sl_list!["Singly", "Linked", "List"];

    assert_eq!(list.get_mut(0), Some(&mut "Singly"));
    assert_eq!(list.get_mut(1), Some(&mut "Linked"));
    assert_eq!(list.get_mut(2), Some(&mut "List"));
    assert_eq!(list.get_mut(3), None);

    assert_eq!(list.get(0), Some(&"Singly"));
    assert_eq!(list.get(1), Some(&"Linked"));
    assert_eq!(list.get(2), Some(&"List"));
    assert_eq!(list.get(3), None);
}


#[test]
fn node_into() {
    let boxed = Node::new(5).into_box();
    let ptr = Node::new(5).into_non_null();
    
    assert_eq!(boxed.value, 5);
    assert_eq!(unsafe { ptr.as_ref().value }, 5);
}