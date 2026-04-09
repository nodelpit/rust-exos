use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let rc_list_1 = Rc::new(List::Cons(10, Rc::new(List::Cons(20, Rc::new(List::Nil)))));
    let _rc_list_2 = Rc::new(List::Cons(3, Rc::clone(&rc_list_1)));
    let _rc_list_3 = Rc::new(List::Cons(5, Rc::clone(&rc_list_1)));

    println!("Quantity of list: {}", Rc::strong_count(&rc_list_1));
}