fn ownership_move() {
    let a = String::from("Hello");
    let b = a;
    // now the heap memory of "Hello" is owned by b
}

fn owning_function() {
    let a = String::from("Hello");
    // receiving_function(a);
    // a.push_str(" World");
    borrowing_function(&a);
    println!("a: {}", a);
}

// fn receiving_function(s: String) {
//     println!("Received: {}", s);
// }

fn borrowing_function(s: &String) {
    println!("Received: {}", s);
}

fn main() {
    println!("Hello, world!");
}
