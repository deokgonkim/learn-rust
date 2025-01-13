fn if_expression(a: i32, b: i32) -> bool {
    let sum = if (a > b) { a + b } else { -1 };
    println!("Sum: {}", sum);
    true
}

fn main() {
    // if_expression(1, 2);
    if_expression(10, 1);
}
