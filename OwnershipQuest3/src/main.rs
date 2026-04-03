use std::ops::Drop;

#[derive(Debug)]
struct Logger {
    name: String,
}

impl Logger {
    fn new(s: &str) -> Logger {
        Logger { name: s.to_string() }
    }
}


impl Drop for Logger {
    fn drop(&mut self) {
        println!("Logger {:?} destroyed", self.name);
    }
}


fn main() {
    let _logger1 = Logger::new("RustLogger");
    let _logger2 = Logger::new("TauriLogger");

    {
        let _logger3 = Logger::new("AxumLogger");
    }
}
