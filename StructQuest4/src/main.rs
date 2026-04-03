#[derive(Debug)]
struct Task {
    title: String,
    description: String,
    state: TaskState,
}

#[derive(Debug, PartialEq)]
enum TaskState {
    Done,
    NotStart,
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl Task {
    fn new(title: &str, description: &str) -> Task {
        Task {
            title: title.to_string(),
            description: description.to_string(),
            state: TaskState::NotStart,
        }
    }

    fn done(&mut self) {
        self.state = TaskState::Done;
    }

    fn is_done(&self) -> bool {
        self.state == TaskState::Done
    }

    fn add_description(&mut self, descr: &str) {
        self.description = descr.to_string();
    }
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn done(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks[index].done();
        } else {
            println!("Index invalide !");
        }
    }

    fn list_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            println!(
                "{}: {} - {} [{:?}]",
                i + 1,
                task.title,
                task.description,
                task.state
            );
        }
    }

    fn list_done_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            if task.is_done() {
                println!(
                    "{}: {} - {} [{:?}]",
                    i + 1,
                    task.title,
                    task.description,
                    task.state
                );
            }
        }
    }

    fn there_remaining_task(&self) -> bool {
        self.tasks.iter().any(|task| !task.is_done())
    }
}

fn main() {
    let mut manager = TaskManager::new();

    let t1 = Task::new("Coder", "Continuer cours Rust");
    let t2 = Task::new("Musique", "Apprendre Ableton");
    let t3 = Task::new("Jeux", "1h Simu FPV");

    manager.add_task(t1);
    manager.add_task(t2);
    manager.add_task(t3);

    println!("---");
    manager.list_tasks();
    println!("---");

    manager.done(0);
    manager.list_done_tasks();
    println!("Reste-t-il des tâches ? {}", manager.there_remaining_task());
}
