use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let t1_borrow = Arc::clone(&counter);
    let t1 = thread::spawn(move || {
        let mut t1_lock = t1_borrow.lock().unwrap();
        *t1_lock += 1;
        println!("{}", t1_lock);
    });

    let t2_borrow = Arc::clone(&counter);
    let t2 = thread::spawn(move || {
        let mut t2_lock = t2_borrow.lock().unwrap();
        *t2_lock += 1;
        println!("{}", t2_lock);
    });

    let t3_borrow = Arc::clone(&counter);
    let t3 = thread::spawn(move || {
        let mut t3_lock = t3_borrow.lock().unwrap();
        *t3_lock += 1;
        println!("{}", t3_lock); 
    });

    let t4_borrow = Arc::clone(&counter);
    let t4 = thread::spawn(move || {
        let mut t4_lock = t4_borrow.lock().unwrap();
        *t4_lock += 1;
        println!("{}", t4_lock); 
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();

    println!("Nombres total d'owner: {:?}", &counter.lock().unwrap());
}
