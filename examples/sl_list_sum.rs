use lists::sl_list;


/// Creates a new `SinglyLinkedList`, and then adds all elements together into a sum.
fn main() {
    let list = sl_list![1, 2, 3, 4, 5];
    let sum = list.into_iter().sum::<i32>();
    
    assert_eq!(sum, 15);
}