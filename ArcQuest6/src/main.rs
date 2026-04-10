use std::sync::{Arc, RwLock};
use std::thread::{self, sleep};
use std::time::Duration;

#[derive(Debug)]
struct Cache {
    data: Vec<String>,
}

impl Cache {
    fn new() -> Cache {
        Cache { data: Vec::new() }
    }

    fn add(&mut self, value: &str) {
        self.data.push(value.to_string());
    } 
}

fn main() {
    let cache = Arc::new(RwLock::new(Cache::new()));

    let mut cache_list = Vec::new();

    for i in 0..=4 {
        let c_borrow = Arc::clone(&cache);

        let cache_thread = thread::spawn(move || {
            let c_read = c_borrow.read().unwrap();

            thread::sleep(Duration::from_millis(500));

            println!("{:?}", c_read.data);
        });

        cache_list.push(cache_thread);
    }
    
    /////////////////

    let cache_borrow = Arc::clone(&cache);
    let cache_thread = thread::spawn(move || {
        let mut c_write = cache_borrow.write().unwrap();

        println!("Writer commence la mise à jour");
        
        c_write.add("Add value");
        println!("{:?}", c_write);
    });

    for thread in cache_list {
        thread.join().expect("Erreur lors de la lecture du cache");
    }
    cache_thread.join().expect("Erreur lors de l'ajout du cache");
}
