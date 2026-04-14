use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

struct Registry {
    data: Arc<RwLock<HashMap<String, i32>>>,
}

impl Registry {
    fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    fn read_all(&self) {
        println!("{:?}", &self.data.read().unwrap());
    }

    fn insert(&self, key: String, value: i32) {
        let mut insertion = self.data.write().unwrap();
        insertion.insert(key, value);
    }
}

fn main() {
    let registry = Arc::new(Registry::new());

    let mut thread_list = Vec::new();

    for _i in 0..=4 {
        let registry_borrow = Arc::clone(&registry);

        let thread = thread::spawn(move || {
            thread::sleep(Duration::from_millis(500));

            registry_borrow.read_all();
        });

        thread_list.push(thread);
    }

    ///////
    let registry_borrow = Arc::clone(&registry);

    let thread = thread::spawn(move || {
        for i in 0..=2 {
            registry_borrow.insert(format!("Key_{}", i), i);
        }
    });

    thread_list.push(thread);

    for t in thread_list {
        t.join().expect("Aucun thread trouvé");
    }
}
