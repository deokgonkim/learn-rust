use std::sync::Mutex;
use std::thread;


fn main() {
    let lock = Mutex::new(5);

    let handle = thread::spawn(move || {
        println!("hello");
        let mut data = lock.lock().unwrap();
        println!("data: {data}");
        *data += 1;
        println!("data: {data}");
    });

    handle.join().unwrap();
}

