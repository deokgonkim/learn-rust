# ownership

- a variable that uses heap spaces, that object's ownership is to that variable's scope
- when a program pointer leaves scope(like {}), the scope level variables heap is released by `Drop`
  ```rust
  {
    let a_string = String::from("Hello");
  } // at this point the heap allocation will be released by runtime
  ```
- when assigning a variable with heap to a new variable, the ownership is transfered. [see ownership_move](../exercises/ownership/src/main.rs)
- when ownership is transfered, the original variable can't be used any more [see owning_function](../exercises/ownership/src/main.rs)
  ```
  error[E0382]: borrow of moved value: `a`
    --> src/main.rs:10:5
       |
    8  |     let mut a = String::from("Hello");
       |         ----- move occurs because `a` has type `String`, which does not implement the `Copy` trait
    9  |     receiving_function(a);
       |                        - value moved here
    10 |     a.push_str(" World");
       |     ^ value borrowed here after move
  ```
  ```
  error[E0382]: borrow of moved value: `a`
    --> src/main.rs:11:23
       |
    8  |     let a = String::from("Hello");
       |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
    9  |     receiving_function(a);
       |                        - value moved here
    10 |     // a.push_str(" World");
    11 |     println!("a: {}", a);
       |                       ^ value borrowed here after move
  ```
- to pass a variable without transfering ownership, we can use `reference` type [see borrowing_function](../exercises/ownership/src/main.rs)
- a primitive typs (i32, char) is allocated to stack instead of heap. so, ownership rule doesn't apply.
