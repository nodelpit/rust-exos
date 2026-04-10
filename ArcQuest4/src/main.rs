use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct LogBook {
    collection: Vec<String>,
}

impl LogBook {
    fn new() -> LogBook {
        LogBook { collection: Vec::new() }
    }

    fn add(&mut self, log: &str) {
        self.collection.push(log.to_string());
    }
}


fn main() {
    let log_book = Arc::new(Mutex::new(LogBook::new()));

    let mut thread_list = Vec::new();

    for i in 1..=4 {
        let log_book_borrow = Arc::clone(&log_book);

        let t = thread::spawn(move || {
            let mut t_lock = log_book_borrow.lock().unwrap();
            t_lock.add(&format!("Worker {} ajouté dans la collection", i));
        });

        thread_list.push(t);
    }

    for thread in thread_list {
        thread.join().expect("Erreur lors de l'ajout de {} dans la collection");
    }

    let log_book_locked = log_book.lock().unwrap();
    println!("Nombres total de logs: {:?}", log_book_locked.collection);
}