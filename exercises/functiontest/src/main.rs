use std::io;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    println!("Enter a number: ");
    io::stdin().read_line(&mut a).expect("Enter a number");
    println!("Enter b number: ");
    io::stdin().read_line(&mut b).expect("Enter b number");
    let a = a.trim().parse().expect("Failed to get a number");
    let b = b.trim().parse().expect("Failed to get b number");
    let sum = add(a, b);
    println!("{a} + {b} = {sum}");
}
