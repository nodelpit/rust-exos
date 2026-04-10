use std::sync::{Arc, RwLock};
use std::thread;

#[derive(Debug)]
struct Config {
    mode: Vec<String>,
}

impl Config {
    fn new() -> Config {
        Config { mode: vec!["Ferry".to_string()] }
    }
}

fn main() {
    let config = Arc::new(RwLock::new(Config::new()));

    let mut thread_list = Vec::new();

    for i in 0..=4 {
        let c_borrow = Arc::clone(&config);

        let t = thread::spawn(move || {
            let c_lock = c_borrow.read().unwrap();
            println!("Thread {}: {:?}", i, c_lock.mode);
        });
        
        thread_list.push(t);
    }

    for thread in thread_list {
        thread.join().unwrap();
    }

    let config_read2 = config.read().unwrap();
    println!("{:?}", config_read2);
}
