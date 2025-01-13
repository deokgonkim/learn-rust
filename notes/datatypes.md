# Data Types

## Tuple

- Similar to tuple in python
- Static sized array data type
- Example
  ```
  (1, 2, 3, 4) // Single data type
  (1, 'a', '😊', "Word")
  ```
- Can have mixed type of elements
- Accessing Element
  ```
  let tuple = (1, 2, 3, 4);
  let first_item = tuple.0;
  ```

## Array

- Similar to array in C
- Static sized array
- Example
  ```
  (1, 2, 3, 4);
  ('a', 'b', 'c', 'd');
  ```
- Cannot have mixed type of element
- Access
  ```
  let array = [1, 2, 3, 4];
  let first_element = array[0];
  ```
- Can be used with `for in`
  ```
  let array = [1, 2, 3, 4];
  for elem in array {
    println!(elem)
  }
  ```
