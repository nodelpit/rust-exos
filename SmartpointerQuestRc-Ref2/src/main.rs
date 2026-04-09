use std::rc::Rc;

fn main() {
    let shared_value1 = Rc::new(10);
    println!("{}", Rc::strong_count(&shared_value1));

    let shared_value2 =  Rc::clone(&shared_value1);
    println!("{}", Rc::strong_count(&shared_value1));

    {
        let shared_value3 = Rc::clone(&shared_value1);
        println!("{}", Rc::strong_count(&shared_value1));
    }

    println!("{}", Rc::strong_count(&shared_value1));
}
