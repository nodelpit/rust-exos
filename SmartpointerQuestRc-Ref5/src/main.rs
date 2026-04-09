use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Log {
    value: String,
}

fn auth_module(param: Rc<RefCell<Vec<Log>>>) {
    param.borrow_mut().push(Log {
        value: "User connected".to_string()
    });
}

fn storage_module(param: Rc<RefCell<Vec<Log>>>) {
    param.borrow_mut().push(Log {
        value: "File uploaded".to_string()
    });
}

fn error_module(param: Rc<RefCell<Vec<Log>>>) {
    param.borrow_mut().push(Log {
        value: "Error: invalid input".to_string()
    });
}

fn main() {
    let logger = Rc::new(RefCell::new(Vec::new()));

    auth_module(Rc::clone(&logger));
    storage_module(Rc::clone(&logger));
    error_module(Rc::clone(&logger));

    println!("Tout les logs: {:?}", logger.borrow());

    for log in logger.borrow().iter() {
        println!("{:?}", log);
    }
}
