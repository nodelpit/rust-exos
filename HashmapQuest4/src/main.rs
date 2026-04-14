use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

struct Cache {
    data: Arc<Mutex<HashMap<String, i32>>>,
}

impl Cache {
    fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

fn main() {
    let cache = Cache::new();

    let mut thread_list = Vec::new();

    for i in 0..=4 {
        let t_borrow = Arc::clone(&cache.data);

        let t = thread::spawn(move || {
            let mut t_lock = t_borrow.lock().unwrap();
            t_lock.insert(format!("Worker_{}", i), i);
        });
        thread_list.push(t);
    }

    for thread in thread_list {
        thread.join().expect("Aucune valeur trouvée");
    }

    println!("{:?}", cache.data.lock().unwrap());
}
