use std::sync::Arc;
use std::thread;

fn main() {
    let s = Arc::new(String::from("mode=debug;version=1.0"));

    let s2_borrow = Arc::clone(&s);
    let s2 = thread::spawn(move || {
        println!("Thread 1: {}", s2_borrow);
    });

    let s3_borrow = Arc::clone(&s);
    let s3 = thread::spawn(move || {
        println!("Thread 2: {}", s3_borrow);
    });

    let s4_borrow = Arc::clone(&s);
    let s4 = thread::spawn(move || {
        println!("Thread 3: {}", s4_borrow);
    });

    s2.join().unwrap();
    s3.join().unwrap();
    s4.join().unwrap();
}
