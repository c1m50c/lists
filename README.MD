# **lists**

![build](https://img.shields.io/github/workflow/status/c1m50c/lists/Build?style=for-the-badge)
![docs.rs](https://img.shields.io/docsrs/lists/latest?style=for-the-badge)
![crates.io](https://img.shields.io/crates/v/lists?color=orange&style=for-the-badge)
![license](https://img.shields.io/crates/l/lists?style=for-the-badge)
![lines of code](https://img.shields.io/tokei/lines/github/c1m50c/lists?style=for-the-badge)
![github stars](https://img.shields.io/github/stars/c1m50c/lists?style=for-the-badge)

Library containing various implementations of list-like data-structures such as `Vectors`, `LinkedLists`, and more.


## **Testing**
```bash
$ cd lists
$ cargo test
...
# If things go well during the tests you should see `ok` as the test result.
```


## **Examples**
<details>
<summary><code>DoublyLinkedList</code> Sum</summary>

```rust
use lists::dl_list;


/// Creates a new `DoublyLinkedList`, and then adds all elements together into a sum.
fn main() {
    let list = dl_list![1, 2, 3, 4, 5];
    let sum = list.into_iter().sum::<i32>();
    
    assert_eq!(sum, 15);
}
```
</details>

<details>
<summary><code>SinglyLinkedList</code> Sum</summary>

```rust
use lists::sl_list;


/// Creates a new `SinglyLinkedList`, and then adds all elements together into a sum.
fn main() {
    let list = sl_list![1, 2, 3, 4, 5];
    let sum = list.into_iter().sum::<i32>();
    
    assert_eq!(sum, 15);
}
```
</details>


## **License**
<a href="https://github.com/c1m50c/lists/blob/main/LICENSE">MIT</a>