//! This program is to test threading and message passing
//!

use std::cell::RefCell;
use std::thread::spawn;
use std::sync::mpsc;

#[derive(Debug)]
struct Message {
    message: String,
    sender: String,
}

impl ToString for Message {
    fn to_string(&self) -> String {
        format!("message: {}, sender: {}", self.message, self.sender)
    }
}


fn main() {
    let (tx, rx) = mpsc::channel();
    let another_tx = tx.clone();

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

    spawn(move || {
        let newMessage = Message {
            message: "Hello".to_string(),
            sender: "This is a new thread".to_string()
        };

        another_tx.send(newMessage.to_string());
    });

    println!("This is main thread");
    loop {
        /*
        let result = rx.recv();
        let received = match result {
            Ok(e) => e,
            Err(e) => panic!("{}", e)
        };
        */
        let received = rx.recv().unwrap();
        println!("Received {:#?}", received);
    }
}

