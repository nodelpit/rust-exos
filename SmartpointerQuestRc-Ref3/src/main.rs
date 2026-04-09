use std::rc::Rc;

enum TaskList {
    Task(String, Rc<TaskList>),
    Empty,
}

impl TaskList {
    fn add_task(self, title: &str) -> TaskList {
        TaskList::Task(title.to_string(), Rc::new(self))
    }
}

fn main() {
    let list = Rc::new(TaskList::Empty);

    let list_1 = Rc::clone(&list);
    let list_2 = Rc::clone(&list);

    println!("Nombres de liste apres ajout: {}", Rc::strong_count(&list));

    {
        let list_temp = Rc::clone(&list);
        println!("Nombres dans le scope: {}", Rc::strong_count(&list));
    }

    println!("Nombres apres scope: {}", Rc::strong_count(&list));
}
