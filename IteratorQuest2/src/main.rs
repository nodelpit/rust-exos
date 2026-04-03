#[derive(Debug)]
struct Task {
    title: String,
    completed: bool,
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl Task {
    fn new(s: &str) -> Task {
        Task {
            title: s.to_string(),
            completed: false
        }
    }

    fn complete(&mut self) {
        self.completed = true
    }
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task)
    }

    fn list_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            println!(
                "{} - {}, {}",
                i + 1,
                task.title,
                task.completed,
            )
        }
    }

    fn completed_tasks(&self) -> Vec<&Task> {
        self.tasks
                .iter()
                .filter(|task| task.completed)
                .collect()
    }
}


fn main() {
    let mut task_manager = TaskManager::new();

    let mut task1 = Task::new("Finir les IteratorQuest");
    let mut task2 = Task::new("Exo complexe Rust");
    let task3 = Task::new("Apprendre Axum");
    let task4 = Task::new("Apprendre Diesel ORM");

    task1.complete();
    task2.complete();

    task_manager.add_task(task1);
    task_manager.add_task(task2);
    task_manager.add_task(task3);
    task_manager.add_task(task4);

    task_manager.list_tasks();

    println!("\n");

    for task in task_manager.completed_tasks() {
        println!("{:?}\n", task);
    }
}