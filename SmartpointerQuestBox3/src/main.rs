#[derive(Debug)]
enum TaskList {
    Task(String, Box<TaskList>),
    Empty,
}

impl TaskList {
    fn new() -> TaskList {
        TaskList::Empty
    }

    fn add(self, title: &str) -> TaskList {
        TaskList::Task(title.to_string(), Box::new(self))
    }
}

fn main() {
    let mut new_task_list = TaskList::new();

    new_task_list = new_task_list.add("Hi");
    new_task_list = new_task_list.add("Rust");
    new_task_list = new_task_list.add("Recursive");
    new_task_list = new_task_list.add("Box");

    println!("\n");

    println!("{:?}", new_task_list);
}
