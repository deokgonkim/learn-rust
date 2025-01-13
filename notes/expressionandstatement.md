# Expression and Statement

## Expression

- a line without semicolon
```
1 + 1
a > 1
```

## Statement

- a line ends with semicolon
```
1 + 1;
```
- line starts with `let` keyword
```
let a = 123;
```

## Other example

- function returning with expression
```
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
- if or loop statement(?) can return statement
```
let sum = if (a > b) { a + b } else { -1 }
```
