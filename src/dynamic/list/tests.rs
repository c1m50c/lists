use super::super::super::list;
use super::List;


#[test]
fn push() {
    let mut list = List::<usize>::new();

    for i in 0 .. 32000 {
        list.push(i);
        assert_eq!(list[i], i);
    }
}


#[test]
fn list_macro() {
    let list = list![1, 2, 3, 4, 5];

    assert_eq!(list.len(), 5);

    for i in 0 .. 5 {
        assert_eq!(list[i], i + 1);
    }
}


#[test]
fn get() {
    let mut list = list![1, 2, 3];

    *list.get_mut(0).unwrap() = 4;
    assert_eq!(list.get(0), Some(&4));
}


#[test]
fn index() {
    let mut list = list![1, 2, 3];

    list[0] = 4;
    list[1] = 5;
    list[2] = 6;

    assert_eq!(list[0], 4);
    assert_eq!(list[1], 5);
    assert_eq!(list[2], 6);
}


#[test]
fn truncate() {
    let mut list = list![3, 2, 1];
    list.truncate(1);

    assert_eq!(list, list![3]);
}


#[test]
fn clear() {
    let mut list = list!["List", "is", "not", "clear"];
    assert_eq!(list.is_empty(), false);

    list.clear();
    assert_eq!(list.is_empty(), true);
}


#[test]
fn eq_ne() {
    let list = list![1, 2, 3, 4, 5];

    assert_eq!(list, list![1, 2, 3, 4, 5]);
    assert_ne!(list, list![0, 1, 2, 3, 4]);
    assert_ne!(list, list![1, 2, 3, 4, 5, 6]);
}