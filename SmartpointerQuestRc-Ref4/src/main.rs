use std::rc::Rc;
use std::cell::RefCell;

struct Counter {
    value: i32,
}

fn module_a(param: Rc<RefCell<Counter>>) {
    param.borrow_mut().value += 1;
}

fn module_b(param: Rc<RefCell<Counter>>) {
    param.borrow_mut().value += 1;
}

fn module_c(param: Rc<RefCell<Counter>>) {
    param.borrow_mut().value += 1;
}

fn main() {
    let counter = Rc::new(RefCell::new(Counter {
        value: 0,
    }));

    module_a(Rc::clone(&counter));
    module_b(Rc::clone(&counter));
    module_c(Rc::clone(&counter));

    println!("{:?}", counter.borrow().value);
}
