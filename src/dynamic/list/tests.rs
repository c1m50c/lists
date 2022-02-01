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