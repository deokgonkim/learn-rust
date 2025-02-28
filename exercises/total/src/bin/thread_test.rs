//! This program is to test threading and message passing
//!

use std::cell::RefCell;
use std::thread::spawn;
use std::sync::mpsc;


fn main() {
    let (tx, rx) = mpsc::channel();

    // `move` is required to use `tx` inside of closure
    spawn(move || {
        let mut string = String::new();
        string.push_str("A new String");
        let anotherString = RefCell::new(string.clone());
        // let anotherString = &string;
        tx.send(string);
        println!("This is in thread");
        println!("Sent {}", anotherString.borrow());
        // println!("Sent {}", anotherString);
    });

    println!("This is main thread");
    let received = rx.recv().unwrap();
    println!("Received {received}");
}

