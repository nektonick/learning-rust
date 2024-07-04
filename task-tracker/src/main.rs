enum TaskStatus {
    Pending,
    Completed,
}

impl TaskStatus {
    fn is_completed(&self) -> bool {
        matches!(self, TaskStatus::Completed)
    }
}

struct Task {
    description: String,
    status: TaskStatus,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            status: TaskStatus::Pending,
        }
    }

    fn complete(&mut self) {
        self.status = TaskStatus::Completed;
    }
}

struct TaskPrinter;

impl TaskPrinter {
    fn print(task: &Task) -> String {
        let status_mark = TaskPrinter::get_status_mark(&task.status);
        format!("- {status_mark} {}", task.description)
    }

    fn get_status_mark(task_status: &TaskStatus) -> &str {
        if task_status.is_completed() {
            "[x]"
        } else {
            "[ ]"
        }
    }
}

struct TaskTracker {
    tasks: Vec<Task>,
    // TODO: task IDs
}

impl TaskTracker {
    fn new() -> TaskTracker {
        TaskTracker { tasks: vec![] }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn list_all_tasks(&self) {
        let all_tasks: Vec<&Task> = self.tasks.iter().collect();
        TaskTracker::list_tasks("All Tasks", &all_tasks);
    }

    fn list_non_completed_tasks(&self) {
        let completed_tasks: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|task| !task.status.is_completed())
            .collect();

        TaskTracker::list_tasks("Non Completed Tasks", &completed_tasks);
    }

    fn list_tasks(header: &str, tasks: &[&Task]) {
        println!("*** {header} ***");
        for task in tasks.iter() {
            println!("{}", TaskPrinter::print(task));
        }
        println!("*** *** ***");
    }

    fn complete_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks[index].complete();
        }
    }
}

fn main() {
    let mut task_tracker = TaskTracker::new();
    task_tracker.add_task(Task::new("Create a Task".to_string()));
    task_tracker.add_task(Task::new("Create a TaskPrinter".to_string()));
    task_tracker.add_task(Task::new("Create a TaskTracker".to_string()));

    task_tracker.list_all_tasks();

    task_tracker.complete_task(0);
    task_tracker.complete_task(2);

    task_tracker.list_all_tasks();
    task_tracker.list_non_completed_tasks();
}
